use std::{
    fs::{FileType, Metadata},
    path::PathBuf,
    str::FromStr,
    time::SystemTime,
};

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

use crate::{
    GeneralFolder, PitouDateTime, PitouDrive, PitouFile, PitouFileFilter, PitouFileKind,
    PitouFileMetadata, PitouFilePath, PitouFileSize, PitouFileSort, PitouTrashItem,
    PitouTrashItemMetadata,
};
use chrono::DateTime;
use trash::TrashItem;

pub mod drive;

pub mod clipboard {
    use std::sync::{Arc, OnceLock};

    use tokio::sync::Mutex;

    pub(crate) enum ClipboardItem {
        Copied(Arc<Vec<PitouFile>>),
        Cut(Arc<Vec<PitouFile>>),
    }

    use crate::PitouFile;
    type QUEUE = Mutex<Vec<ClipboardItem>>;

    static CLIPBOARD: OnceLock<QUEUE> = OnceLock::new();

    fn get_clipboard() -> &'static QUEUE {
        CLIPBOARD.get_or_init(|| Mutex::new(Vec::new()))
    }

    pub(super) async fn copy(files: Vec<PitouFile>) {
        get_clipboard()
            .lock()
            .await
            .push(ClipboardItem::Copied(Arc::new(files)))
    }

    pub(super) async fn cut(files: Vec<PitouFile>) {
        get_clipboard()
            .lock()
            .await
            .push(ClipboardItem::Cut(Arc::new(files)))
    }

    pub async fn remove_from_clipboard(idx: usize) {
        get_clipboard().lock().await.remove(idx);
    }

    pub async fn clear_clipboard() {
        get_clipboard().lock().await.clear()
    }

    pub async fn is_empty() -> bool {
        get_clipboard().lock().await.is_empty()
    }

    pub(crate) async fn paste() -> Option<ClipboardItem> {
        let cb = get_clipboard();
        let mut guard = cb.lock().await;
        let items = guard.pop();
        match &items {
            None => (),
            Some(v) => match v {
                ClipboardItem::Copied(u) => guard.push(ClipboardItem::Copied(u.clone())),
                ClipboardItem::Cut(u) => guard.push(ClipboardItem::Copied(u.clone())),
            },
        }
        std::mem::drop(guard);
        items
    }
}

pub fn drives() -> Vec<PitouDrive> {
    let mut drives = PitouDrive::get_drives();
    drives.sort_unstable_by(|a, b| a.mount_point.name().cmp(b.mount_point.name()));
    drives
}

pub fn delete(items: Vec<PitouFile>) {
    for item in items {
        tokio::spawn(async move { trash::delete(&item.path.path) });
    }
}

pub async fn copy(items: Vec<PitouFile>) {
    clipboard::copy(items).await
}

pub async fn cut(items: Vec<PitouFile>) {
    clipboard::cut(items).await
}

pub fn open(file: PitouFilePath) -> std::io::Result<()> {
    open::that_detached(&file.path)
}

pub fn open_with(file: PitouFilePath) -> Result<(), ()> {
    open_with::open_with(file.path).map_err(|_| ())
}

pub fn share(_file: PitouFilePath) -> std::io::Result<()> {
    todo!()
}

pub async fn rename(file: PitouFilePath, newname: String) {
    let newpath = file.path.parent().unwrap_or(&PathBuf::new()).join(newname);
    tokio::fs::rename(&file.path, newpath).await.unwrap();
}

pub async fn create_file(file: PitouFilePath) {
    tokio::fs::File::create(&file.path)
        .await
        .expect("couldn't create file");
}

pub async fn create_dir(dir: PitouFilePath) {
    tokio::fs::create_dir(&dir.path)
        .await
        .expect("couldn't create dir");
}

pub async fn read_link(link: PitouFilePath) -> Option<crate::PitouFile> {
    tokio::fs::read_link(&link.path)
        .await
        .map(|path| PitouFile::from_pathbuf(path))
        .ok()
}

pub async fn children(
    dir: PitouFilePath,
    filter: PitouFileFilter,
    sort: Option<PitouFileSort>,
) -> std::io::Result<Vec<PitouFile>> {
    if dir.path.as_os_str().len() == 0 {
        let items = PitouDrive::get_drives()
            .into_iter()
            .filter_map(|drive| filter.map(PitouFile::from_pathbuf(drive.mount_point.path)))
            .collect::<Vec<_>>();
        return if let Some(sort) = sort {
            Ok(sort.sorted(items))
        } else {
            Ok(items)
        };
    }

    let mut read_dir = tokio::fs::read_dir(&dir.path).await?;
    let mut res = Vec::new();
    while let Some(entry) = read_dir.next_entry().await? {
        let file = PitouFile::from_pathbuf(entry.path());
        if let Some(file) = filter.map(file) {
            res.push(file);
        }
    }
    return if let Some(sort) = sort {
        Ok(sort.sorted(res))
    } else {
        Ok(res)
    };
}

impl PitouFile {
    pub fn from_pathbuf(path: PathBuf) -> Self {
        let mut metadata = PitouFileMetadata::attempt(&path);
        if let Some(metadata) = &mut metadata {
            if metadata.is_dir() {
                metadata.size = Self::attempt_count(&path).into();
            }
        }
        Self {
            metadata,
            path: path.into(),
        }
    }

    fn attempt_count(path: &PathBuf) -> u64 {
        let mut count = 0;
        if let Ok(mut read_dir) = std::fs::read_dir(path) {
            while let Some(_) = read_dir.next() {
                count += 1;
            }
        }
        count
    }
}

impl PitouFileMetadata {
    fn attempt(path: &PathBuf) -> Option<Self> {
        std::fs::metadata(path).map(|v| v.into()).ok()
    }
}

pub async fn siblings(
    mut dir: PitouFilePath,
    filter: PitouFileFilter,
    sort: Option<PitouFileSort>,
) -> std::io::Result<Vec<PitouFile>> {
    dir.path.pop();
    children(dir, filter, sort).await
}

pub fn default_folder() -> PitouFile {
    let path = PitouFilePath::from_pathbuf(dirs::home_dir().unwrap());
    PitouFile {
        path,
        metadata: None,
    }
}

fn downloads_folder() -> PitouFilePath {
    PitouFilePath::from_pathbuf(dirs::download_dir().unwrap())
}

fn desktop_folder() -> PitouFilePath {
    PitouFilePath::from_pathbuf(dirs::desktop_dir().unwrap())
}

fn videos_folder() -> PitouFilePath {
    PitouFilePath::from_pathbuf(dirs::video_dir().unwrap())
}

fn pictures_folder() -> PitouFilePath {
    PitouFilePath::from_pathbuf(dirs::picture_dir().unwrap())
}

fn audios_folder() -> PitouFilePath {
    PitouFilePath::from_pathbuf(dirs::audio_dir().unwrap())
}

fn documents_folder() -> PitouFilePath {
    PitouFilePath::from_pathbuf(dirs::document_dir().unwrap())
}

pub fn general_folders() -> Vec<GeneralFolder> {
    vec![
        GeneralFolder::DesktopFolder(desktop_folder()),
        GeneralFolder::DownloadsFolder(downloads_folder()),
        GeneralFolder::AudiosFolder(audios_folder()),
        GeneralFolder::VideosFolder(videos_folder()),
        GeneralFolder::PicturesFolder(pictures_folder()),
        GeneralFolder::DocumentsFolder(documents_folder()),
    ]
}

pub fn trash_items() -> Option<Vec<PitouTrashItem>> {
    trash::os_limited::list()
        .map(|v| v.into_iter().filter_map(|u| u.try_into().ok()).collect())
        .ok()
}

pub fn restore_trash(items: impl Iterator<Item = PitouTrashItem>) {
    trash::os_limited::restore_all(items.map(|v| <PitouTrashItem as Into<TrashItem>>::into(v)))
        .ok();
}

pub fn purge_trash(items: impl Iterator<Item = PitouTrashItem>) {
    trash::os_limited::purge_all(items.map(|v| <PitouTrashItem as Into<TrashItem>>::into(v))).ok();
}

impl From<PitouTrashItem> for TrashItem {
    fn from(
        PitouTrashItem {
            mut original_path,
            metadata:
                PitouTrashItemMetadata {
                    id,
                    deleted,
                    size: _,
                    is_dir: _,
                },
        }: PitouTrashItem,
    ) -> Self {
        let time_deleted = deleted.datetime.and_utc().timestamp();
        let name = original_path.name().to_owned();
        original_path.path.pop();
        let original_parent = original_path.path;

        Self {
            id: std::ffi::OsString::from_str(&id).unwrap(),
            name,
            time_deleted,
            original_parent,
        }
    }
}

impl TryFrom<TrashItem> for PitouTrashItem {
    type Error = trash::Error;
    fn try_from(item: TrashItem) -> Result<Self, Self::Error> {
        let (size, is_dir) = match trash::os_limited::metadata(&item)?.size {
            trash::TrashItemSize::Bytes(val) => (val, false),
            trash::TrashItemSize::Entries(val) => (val as u64, true),
        };

        let TrashItem {
            id,
            name,
            mut original_parent,
            time_deleted,
        } = item;

        original_parent.push(name);

        let metadata = PitouTrashItemMetadata {
            id: id.into_string().unwrap(),
            deleted: PitouDateTime {
                datetime: DateTime::from_timestamp_millis(1000 * time_deleted)
                    .unwrap()
                    .naive_utc(),
            },
            is_dir,
            size: PitouFileSize::new(size),
        };

        Ok(PitouTrashItem {
            original_path: PitouFilePath::from_pathbuf(original_parent),
            metadata,
        })
    }
}

impl From<FileType> for PitouFileKind {
    fn from(value: FileType) -> Self {
        if value.is_dir() {
            Self::Directory
        } else if value.is_file() {
            Self::File
        } else {
            Self::Link
        }
    }
}

impl PitouFile {
    pub fn new(path: PathBuf, metadata: Metadata) -> Self {
        let path = path.into();
        let metadata = metadata.try_into().ok();
        Self { path, metadata }
    }
}

impl From<SystemTime> for PitouDateTime {
    fn from(value: SystemTime) -> Self {
        let millis_epoch = value
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        Self {
            datetime: DateTime::from_timestamp_millis(millis_epoch)
                .unwrap()
                .naive_utc(),
        }
    }
}

impl From<Metadata> for PitouFileMetadata {
    fn from(value: Metadata) -> Self {
        #[cfg(target_os = "windows")]
        let attribute = value.file_attributes();
        #[cfg(not(target_os = "windows"))]
        let attribute = 0u32; // Default value for non-Windows platforms
        
        Self {
            modified: value.modified().unwrap().into(),
            accessed: value.accessed().unwrap().into(),
            created: value.created().unwrap().into(),
            size: value.len().into(),
            kind: value.file_type().into(),
            attribute,
        }
    }
}
