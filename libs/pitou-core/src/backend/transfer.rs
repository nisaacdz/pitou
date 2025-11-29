use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
    thread,
    time::Instant,
};

use crate::{
    msg::{TransferMsg, TransferSessionID, TransferSize, TransferState},
    PitouFile, PitouFilePath,
};

use super::clipboard;

impl TransferState {
    /// Adds the supplied value to the current size. This method automatically checks if the transfer is completed changes the state from Active to Terminated
    ///
    /// A hypothetical problem with this approach it that if a number of folders and some number of files are being copied, and the files somehow finish getting copied before the folder,
    /// this operation will change the state of the transfer to Terminated.
    ///
    /// A simple fix is to pretend the folders also have size. This will be used in both the total and current computations.
    fn append_current(&mut self, val: u64) {
        if let Self::Active(TransferSize { total: _, current }) = self {
            *current += val;
            /*
            if *current == *total {
                *self = TransferState::Terminated(TransferSize { total: *total, current: *current })
            }
            */
        }
    }

    fn append_total(&mut self, val: u64) {
        if let Self::Initializing(total) = self {
            *total += val
        }
    }

    fn end_init(&mut self) {
        if let Self::Initializing(total) = *self {
            let current = 0;
            *self = Self::Active(TransferSize { total, current })
        }
    }
}

struct TransferConfig {
    id: TransferSessionID,
    state: Mutex<TransferState>,
    started: Mutex<Instant>,
    copy: bool,
}

impl TransferConfig {
    fn is_ongoing(&self) -> bool {
        !matches!(*self.state.lock().unwrap(), TransferState::Terminated(_))
    }

    fn start_now(&self) {
        *self.started.lock().unwrap() = Instant::now()
    }

    fn terminate_now(&self) {
        let mut state = self.state.lock().unwrap();
        if let TransferState::Active(sz) = *state {
            *state = TransferState::Terminated(sz)
        }
    }

    fn begin_transfer(self: &Arc<Self>, items: Arc<Vec<PitouFile>>, dst: PitouFilePath) {
        let config = self.clone();
        let copy = self.copy;
        let _ = thread::spawn(move || {
            if copy {
                AllItemsCopySesssion::init(config, items, dst)
            } else {
                AllItemsCopySesssion::init(config, items, dst)
            }
        });
    }

    fn read(&self) -> TransferMsg {
        let state = *self.state.lock().unwrap();
        let id = self.id;
        let time_elapsed = self.started.lock().unwrap().elapsed();

        if self.copy {
            TransferMsg::Copy {
                id,
                state,
                time_elapsed,
            }
        } else {
            TransferMsg::Move {
                id,
                state,
                time_elapsed,
            }
        }
    }
}

const TRANSFER_BUFFER_SIZE: usize = 1024;
const HYPOTHETICAL_FOLDER_SIZE: u64 = 1;
type CONFIGURATIONS = Mutex<Vec<Arc<TransferConfig>>>;
static SESSIONS: OnceLock<CONFIGURATIONS> = OnceLock::new();

fn get_sessions() -> &'static CONFIGURATIONS {
    SESSIONS.get_or_init(|| Mutex::new(Vec::new()))
}

fn add_new_session(copy: bool) -> Arc<TransferConfig> {
    let config = Arc::new(TransferConfig {
        id: generate_id(),
        state: Mutex::new(TransferState::Initializing(0)),
        started: Mutex::new(Instant::now()),
        copy,
    });
    get_sessions().lock().unwrap().push(config.clone());
    config
}

fn generate_id() -> TransferSessionID {
    TransferSessionID {
        idx: get_sessions().lock().unwrap().len() as i64,
        parity: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as _,
    }
}

fn dst_temp(src: &PathBuf, dst: &PathBuf) -> PathBuf {
    let name = src.file_name().unwrap();
    dst.join(format!(".{}", name.to_str().unwrap()))
}

fn dst_real(src: &PathBuf, dst: &PathBuf) -> PathBuf {
    let name = src.file_name().unwrap();
    dst.join(name)
}

pub fn get_all_active_sessions() -> Vec<TransferMsg> {
    get_sessions()
        .lock()
        .unwrap()
        .iter()
        .filter_map(|v| if v.is_ongoing() { Some(v.read()) } else { None })
        .collect()
}

pub fn get_session_with_id(id: TransferSessionID) -> Option<TransferMsg> {
    let idx = id.idx as usize;
    get_sessions()
        .lock()
        .unwrap()
        .get(idx)
        .map(|v| {
            if id.parity == v.id.parity {
                Some(v.read())
            } else {
                None
            }
        })
        .flatten()
}

pub fn clean_dead_sessions() {
    let mut sessions = get_sessions().lock().unwrap();
    let new_sessions = sessions
        .iter()
        .filter_map(|v| {
            if v.is_ongoing() {
                Some(v.clone())
            } else {
                None
            }
        })
        .collect();
    let _ = std::mem::replace(&mut *sessions, new_sessions);
}

pub async fn paste_items(dst: PitouFilePath) -> Option<TransferSessionID> {
    match clipboard::paste().await {
        None => None,
        Some(v) => match v {
            clipboard::ClipboardItem::Copied(items) => {
                let config = add_new_session(true);
                config.begin_transfer(items, dst);
                Some(config.id)
            }
            clipboard::ClipboardItem::Cut(items) => {
                let config = add_new_session(false);
                config.begin_transfer(items, dst);
                Some(config.id)
            }
        },
    }
}

#[cfg(test)]
mod test_mod {
    use crate::PitouFileSize;

    use super::*;
    use tokio_stream::{wrappers::IntervalStream, StreamExt};
    #[test]
    fn test_copy() {
        // Run: cargo test -p pitou-core --lib -- backend::transfer::test_mod::test_copy --nocapture
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            let src_path = PathBuf::from("C:\\Users\\nisaacdz\\Desktop\\Tmp\\Tmp2\\DCIT 103");
            let dst_path = PathBuf::from("C:\\Users\\nisaacdz\\Desktop\\Tmp\\Tmp2\\PasteHere");

            let items = vec![PitouFile::without_metadata(PitouFilePath::from_pathbuf(src_path))];
            super::super::copy(items).await;
            if let Some(session_id) = super::paste_items(PitouFilePath::from_pathbuf(dst_path)).await {
                let mut interval = IntervalStream::new(tokio::time::interval(std::time::Duration::from_millis(500)));
                while let Some(_) = interval.next().await {
                    let msg = get_session_with_id(session_id).unwrap();
                    let (state, duration) = msg.details();
                        match state {
                            TransferState::Initializing(c) => println!{"Computing size: {c}"},
                            TransferState::Active(TransferSize { total, current }) => {
                                let elapsed = duration.as_secs_f64();
                                let size_msg = format!{"completed {} of {}", PitouFileSize::new(current).format(), PitouFileSize::new(total).format()};
                                let estimated_time_rem = (elapsed * total as f64 / current as f64) - elapsed;

                                println!("{size_msg} | time-spent: {:.2}s | estimated-time-rem: {:.2}s", elapsed, estimated_time_rem)
                            },
                            TransferState::Terminated(TransferSize { total, current }) => {
                                let elapsed = duration.as_secs_f64();
                                println!{"{} of {} | total-time-spent: {:.2}s", PitouFileSize::new(current).format(), PitouFileSize::new(total).format(), elapsed};
                                break;
                            }
                        }
                }
            } else {
                println!("nothing on clipboard to copypaste");
            }
        });
    }
}

struct AllItemsCopySesssion {
    config: Arc<TransferConfig>,
    items: Vec<Arc<PathBuf>>,
    dst: Arc<PathBuf>,
}

impl AllItemsCopySesssion {
    fn init(config: Arc<TransferConfig>, items: Arc<Vec<PitouFile>>, dst: PitouFilePath) {
        let config2 = config.clone();
        let items = std::thread::scope(move |s| {
            let mut handles = Vec::with_capacity(items.len());
            for item in items.iter() {
                let config = config.clone();
                let path = item.path.path.clone();
                let shc = s.spawn(move || Self::compute_size(path, config));
                handles.push(shc)
            }
            handles
                .into_iter()
                .map(|v| Arc::new(v.join().unwrap()))
                .collect::<Vec<_>>()
        });
        let config = config2;
        config.state.lock().unwrap().end_init();
        let session = Self {
            config,
            items,
            dst: Arc::new(dst.path),
        };

        session.config.start_now();

        session.proceed();
    }

    fn proceed(self) {
        let Self { config, items, dst } = self;
        let mut handles = Vec::with_capacity(items.len());
        for item in items {
            let config = config.clone();
            let dst = dst.clone();
            let hdl = std::thread::spawn(move || {
                if item.is_dir() {
                    CopyFolderSession::new(config, (*item).clone(), dst)
                        .proceed()
                        .unwrap();
                } else {
                    CopyFileSession::new(config, (*item).clone(), dst)
                        .unwrap()
                        .proceed()
                        .unwrap();
                }
            });
            handles.push(hdl);
        }

        thread::spawn(move || {
            for handle in handles {
                handle.join().unwrap();
            }
            config.terminate_now();
        });
    }

    fn compute_size(item: PathBuf, config: Arc<TransferConfig>) -> PathBuf {
        let mut size = 0;
        let q = std::fs::metadata(&item).unwrap();
        if q.is_dir() {
            size += HYPOTHETICAL_FOLDER_SIZE;
            let mut rd = std::fs::read_dir(&item).unwrap();
            while let Some(de) = rd.next() {
                let entry = de.unwrap();
                Self::compute_size(entry.path(), config.clone());
            }
        } else {
            size += q.len();
        }
        config.state.lock().unwrap().append_total(size);
        item
    }
}

struct CopyFolderSession {
    config: Arc<TransferConfig>,
    src_folder: PathBuf,
    dst_folder: Arc<PathBuf>,
}

impl CopyFolderSession {
    fn new(config: Arc<TransferConfig>, src_folder: PathBuf, dst_folder: Arc<PathBuf>) -> Self {
        Self {
            config,
            src_folder,
            dst_folder,
        }
    }

    fn proceed(self) -> Result<(), std::io::Error> {
        let Self {
            config,
            src_folder,
            dst_folder,
        } = self;

        let mut rd = std::fs::read_dir(&src_folder)?;
        let dst_folder = Arc::new(dst_real(&src_folder, &dst_folder));
        std::fs::create_dir(&*dst_folder)?;
        while let Some(en) = rd.next() {
            let elem = en?.path();
            if elem.is_dir() {
                Self::new(config.clone(), elem, dst_folder.clone()).proceed()?;
            } else {
                CopyFileSession::new(config.clone(), elem, dst_folder.clone())?.proceed()?;
            }
        }
        config
            .state
            .lock()
            .unwrap()
            .append_current(HYPOTHETICAL_FOLDER_SIZE);
        Ok(())
    }
}

struct CopyFileSession {
    src_file: File,
    dst_file: File,
    temp_dst_path: PathBuf,
    real_dst_path: PathBuf,
    seek_ptr: u64,
    config: Arc<TransferConfig>,
}

impl CopyFileSession {
    fn new(config: Arc<TransferConfig>, src: PathBuf, dst: Arc<PathBuf>) -> std::io::Result<Self> {
        let temp_dst_path = dst_temp(&src, &dst);
        let real_dst_path = dst_real(&src, &dst);
        let src_file = File::open(&src)?;
        let dst_file = File::create(&temp_dst_path)?;

        Ok(Self {
            src_file,
            temp_dst_path,
            dst_file,
            seek_ptr: 0,
            config,
            real_dst_path,
        })
    }

    fn proceed(&mut self) -> Result<(), std::io::Error> {
        self.src_file.seek(SeekFrom::Start(self.seek_ptr))?;
        let mut buffer = vec![0; TRANSFER_BUFFER_SIZE];
        while self.seek_ptr < self.src_file.metadata()?.len() {
            let cnt = self.src_file.read(&mut buffer)?;
            self.dst_file.write_all(&buffer[..cnt])?;
            self.seek_ptr += cnt as u64;
            self.src_file.seek(SeekFrom::Start(self.seek_ptr))?;
            self.config.state.lock().unwrap().append_current(cnt as u64);
        }
        std::fs::rename(&self.temp_dst_path, &*self.real_dst_path)
    }
}
