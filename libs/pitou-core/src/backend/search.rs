use std::{path::PathBuf, sync::Arc};

use crate::{search::SimplifiedSearchOptions, PitouFile, PitouFileFilter};

impl SimplifiedSearchOptions {
    pub fn try_into(self) -> Option<SearchOptions> {
        if let Some(search_type) = SearchType::parse_if_regex(self.search_kind, self.input) {
            let obj = SearchOptions {
                search_dir: self.search_dir,
                filter: self.filter,
                case_sensitive: self.case_sensitive,
                hardware_accelerate: self.hardware_accelerate,
                skip_errors: self.skip_errors,
                depth: self.depth,
                max_finds: self.max_finds,
                search_type: search_type,
            };
            Some(obj)
        } else {
            None
        }
    }
}

impl SearchType {
    pub(crate) fn parse_if_regex(search_kind: u8, search_key: String) -> Option<Self> {
        match search_kind {
            0 => regex::Regex::new(&search_key)
                .map(|r| SearchType::Regex(r))
                .ok(),
            1 => Some(SearchType::MatchBegining(search_key)),
            2 => Some(SearchType::MatchEnding(search_key)),
            3 => Some(SearchType::MatchMiddle(search_key)),
            _ => None,
        }
    }
}

pub struct SearchOptions {
    pub(crate) search_dir: PitouFile,
    pub(crate) hardware_accelerate: bool,
    pub(crate) filter: PitouFileFilter,
    pub(crate) case_sensitive: bool,
    pub(crate) depth: u8,
    pub(crate) search_type: SearchType,
    pub(crate) skip_errors: bool,
    pub(crate) max_finds: usize,
}

impl SearchType {
    pub(crate) fn matches(&self, input: &str, sensitive: bool) -> bool {
        match self {
            Self::Regex(pattern) => pattern.is_match(input),
            Self::MatchBegining(key) => {
                if sensitive {
                    input.starts_with(key)
                } else {
                    crate::extra::starts_with_ignore_case(key, input)
                }
            }
            Self::MatchMiddle(key) => {
                if sensitive {
                    input.contains(key)
                } else {
                    crate::extra::contains_ignore_case(key, input)
                }
            }
            Self::MatchEnding(key) => {
                if sensitive {
                    input.ends_with(key)
                } else {
                    crate::extra::ends_with_ignore_case(key, input)
                }
            }
        }
    }
}

mod stream {
    use std::{collections::LinkedList, sync::OnceLock};

    use crate::{msg::SearchMsg, PitouFile};
    use tokio::{sync::Mutex, task::JoinHandle};

    type COUNT = Mutex<usize>;
    type QUEUE = Mutex<Option<LinkedList<PitouFile>>>;
    type SPAWNS = Mutex<LinkedList<JoinHandle<()>>>;

    static HANDLES: OnceLock<SPAWNS> = OnceLock::new();
    static STREAM: OnceLock<QUEUE> = OnceLock::new();
    static FINDS: OnceLock<COUNT> = OnceLock::new();

    fn get_finds() -> &'static COUNT {
        FINDS.get_or_init(|| Mutex::new(0))
    }

    fn get_handles() -> &'static SPAWNS {
        HANDLES.get_or_init(|| Mutex::new(LinkedList::new()))
    }

    fn get_stream() -> &'static QUEUE {
        STREAM.get_or_init(|| Mutex::new(None))
    }

    /// decrements the count and returns true if the max_finds has not yet been exhusted
    /// Automatically closes the finds if the count has dropped to zero.
    async fn count_and_proceed() -> bool {
        let mut c_val = get_finds().lock().await;
        if *c_val == 0 {
            false
        } else {
            *c_val -= 1;
            true
        }
    }

    pub(super) async fn terminate_stream() {
        *get_finds().lock().await = 0;
    }

    /// checks if the stream is terminated
    pub(super) async fn is_terminated() -> bool {
        *get_finds().lock().await == 0
    }

    pub(super) async fn proceed_to_finish_stream() {
        loop {
            let mut hlock = get_handles().lock().await;
            if hlock.is_empty() {
                break;
            }
            let handles = hlock.split_off(0);
            std::mem::drop(hlock);
            for handle in handles.into_iter().rev() {
                let _ = handle.await;
            }
        }
        terminate_stream().await;
    }

    pub(super) async fn configure_stream(max_finds: usize) {
        tokio::join! {
            async move { let _ = get_stream().lock().await.insert(LinkedList::new()); },
            async move { *get_finds().lock().await = max_finds },
            async move { let _ = get_handles().lock().await.clear(); }
        };
    }

    pub(super) async fn read() -> SearchMsg {
        if is_terminated().await {
            get_stream()
                .lock()
                .await
                .as_mut()
                .map(|l| SearchMsg::Terminated(l.split_off(0)))
                .unwrap_or(SearchMsg::Terminated(LinkedList::new()))
        } else {
            get_stream()
                .lock()
                .await
                .as_mut()
                .map(|l| SearchMsg::Active(l.split_off(0)))
                .unwrap_or(SearchMsg::Terminated(LinkedList::new()))
        }
    }

    pub(super) async fn write(find: PitouFile) {
        if count_and_proceed().await {
            get_stream()
                .lock()
                .await
                .as_mut()
                .map(|l| l.push_back(find));
        } else {
            abort_remaining_ops().await
        }
    }

    pub(super) async fn append_handle(handle: JoinHandle<()>) {
        get_handles().lock().await.push_back(handle);
    }

    pub(super) async fn abort_remaining_ops() {
        let mut handles = get_handles().lock().await;
        for handle in handles.split_off(0).into_iter().rev() {
            handle.abort()
        }
        std::mem::drop(handles);
    }

    pub(super) async fn wait_for_all_ops() {
        ()
    }
}

#[allow(unused)]
#[derive(Clone)]
struct SearchVariables {
    filter: PitouFileFilter,
    case_sensitive: bool,
    depth: u8,
    search_type: Arc<SearchType>,
    skip_errors: bool,
}

impl From<SearchOptions> for (SearchVariables, PathBuf) {
    fn from(value: SearchOptions) -> Self {
        let SearchOptions {
            search_dir,
            hardware_accelerate: _,
            filter,
            case_sensitive,
            depth,
            search_type,
            skip_errors,
            max_finds: _,
        } = value;
        (
            SearchVariables {
                filter,
                case_sensitive,
                depth,
                skip_errors,
                search_type: Arc::new(search_type),
            },
            search_dir.path.path,
        )
    }
}

impl SearchVariables {
    fn include(&self, file: &PitouFile) -> bool {
        ((file.is_file() && self.filter.files)
            || (file.is_dir() && self.filter.dirs)
            || (file.is_link() && self.filter.links)
            || (file.is_sys_item() && self.filter.sys_items))
            && self.search_type.matches(file.name(), self.case_sensitive)
    }
}

pub async fn read_stream() -> crate::msg::SearchMsg {
    stream::read().await
}

pub async fn terminate_search() {
    stream::terminate_stream().await
}

pub async fn is_terminated() -> bool {
    stream::is_terminated().await
}

pub async fn search(options: SearchOptions) {
    let hardware_accelerate = options.hardware_accelerate;
    let max_finds = options.max_finds;
    let (variables, directory) = options.into();
    if variables.filter.all_filtered() {
        return;
    }
    stream::configure_stream(max_finds).await;
    tokio::spawn(async move {
        recursive_search(directory, variables).await;
        stream::proceed_to_finish_stream().await;
        if hardware_accelerate {
            stream::wait_for_all_ops().await;
        }
    });
}

#[async_recursion::async_recursion]
async fn recursive_search(directory: PathBuf, mut variables: SearchVariables) {
    if variables.depth == 0 || stream::is_terminated().await {
        return;
    }
    variables.depth -= 1;
    let mut read_dir = if let Ok(read_dir) = tokio::fs::read_dir(&directory).await {
        read_dir
    } else {
        return;
    };

    while let Ok(Some(de)) = read_dir.next_entry().await {
        let file = PitouFile::new(de.path(), de.metadata().await.unwrap());
        if file.is_dir() {
            let vclone = variables.clone();
            stream::append_handle(tokio::spawn(async move {
                recursive_search(de.path(), vclone).await
            }))
            .await;
        }
        if variables.include(&file) {
            stream::write(file).await;
        }
    }
}

#[derive(Clone)]
pub enum SearchType {
    Regex(regex::Regex),
    MatchBegining(String),
    MatchMiddle(String),
    MatchEnding(String),
}
