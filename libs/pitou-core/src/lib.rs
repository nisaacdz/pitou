use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::{cmp::Reverse, path::PathBuf, rc::Rc};
pub mod extra;

#[cfg(feature = "frontend")]
pub mod frontend;

#[cfg(feature = "backend")]
pub mod backend;

pub mod collections;
pub mod msg;
pub mod search;

pub(crate) mod ser_de;

/// Custom file type which is just a wrapper around the std `PathBuf` for cross-platform serialization and deserialization.
#[derive(PartialEq)]
pub struct PitouFilePath {
    pub path: PathBuf,
}

impl PitouFilePath {
    pub fn name(&self) -> &str {
        if self.path.as_os_str().len() == 0 {
            return "Drives";
        }
        let res = self
            .path
            .file_name()
            .map(|v| v.to_str().unwrap_or_default())
            .unwrap_or_default();
        res
    }

    pub fn extension(&self) -> &str {
        self.path
            .extension()
            .map(|v| v.to_str().unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn as_os_str(&self) -> &std::ffi::OsStr {
        self.path.as_os_str()
    }

    pub fn from_pathbuf(pathbuf: PathBuf) -> Self {
        Self { path: pathbuf }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.path.as_os_str().as_encoded_bytes()
    }

    pub fn ancestors(&self) -> impl Iterator<Item = PitouFilePath> {
        let mut ll = std::collections::LinkedList::new();
        for anc in self.path.ancestors() {
            ll.push_front(PitouFilePath::from_pathbuf(std::path::PathBuf::from(anc)))
        }
        ll.into_iter()
    }

    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct PitouDateTime {
    pub datetime: NaiveDateTime,
}

impl PitouDateTime {
    pub fn format_duration(duration: std::time::Duration) -> String {
        const MINS: u64 = 60;
        const HRS: u64 = 60 * 60;
        const DAYS: u64 = 24 * 60 * 60;

        let time = duration.as_secs();
        if time <= 2 * MINS {
            return format! {"{} secs", time};
        }
        let mins_time = time / MINS;
        if mins_time <= 2 * HRS {
            return format! {"{} mins", mins_time};
        }
        let hrs_time = time / HRS;
        if hrs_time <= 2 * DAYS {
            return format! {"{} hrs", hrs_time};
        }

        let days_time = time / DAYS;

        return format! {"{} days", days_time};
    }
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum PitouFileKind {
    Directory,
    File,
    Link,
}

#[derive(Clone, Copy)]
pub struct PitouFileSize {
    pub bytes: u64,
}

impl PitouFileSize {
    const KB: f64 = (1u64 << 10) as f64;
    const MB: f64 = (1u64 << 20) as f64;
    const GB: f64 = (1u64 << 30) as f64;
    const TB: f64 = (1u64 << 40) as f64;
    pub fn format(self) -> String {
        let bytes = self.bytes as f64;
        if bytes < Self::KB {
            format! {"{} B", bytes}
        } else if bytes < Self::MB {
            format! {"{:.2} KB", bytes / Self::KB }
        } else if bytes < Self::GB {
            format! {"{:.2} MB", bytes / Self::MB }
        } else if bytes < Self::TB {
            format! {"{:.2} GB", bytes / Self::GB }
        } else {
            format! {"{:.2} TB", bytes / Self::TB }
        }
    }

    pub fn new(value: u64) -> Self {
        Self { bytes: value }
    }

    pub fn format_as_dir_entries(&self) -> String {
        if self.bytes == 0 {
            "Empty".to_owned()
        } else if self.bytes == 1 {
            "1 item".to_owned()
        } else {
            format!("{} items", self.bytes)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PitouFileMetadata {
    pub modified: PitouDateTime,
    pub accessed: PitouDateTime,
    pub created: PitouDateTime,
    pub size: PitouFileSize,
    pub kind: PitouFileKind,
    pub attribute: u32,
}

impl PitouFileMetadata {
    pub fn is_dir(&self) -> bool {
        matches!(self.kind, PitouFileKind::Directory)
    }

    pub fn kind(&self) -> PitouFileKind {
        self.kind
    }

    pub fn is_sys_item(&self) -> bool {
        match self.attribute {
            2 => true,
            256 => true,
            _ => false,
        }
    }
}

pub struct PitouDrive {
    pub name: String,
    pub mount_point: PitouFilePath,
    pub total_space: u64,
    pub free_space: u64,
    pub is_removable: bool,
    pub kind: PitouDriveKind,
}

impl PitouDrive {
    pub fn mount_point(&self) -> &PitouFilePath {
        &self.mount_point
    }

    pub fn as_pitou_file(&self) -> PitouFile {
        let path = self.mount_point().path.clone().into();
        PitouFile::without_metadata(path)
    }
}

impl PartialEq for PitouDrive {
    fn eq(&self, other: &Self) -> bool {
        &self.mount_point == &other.mount_point
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PitouDriveKind {
    HDD,
    SSD,
    Unknown,
}

pub struct PitouFile {
    pub path: PitouFilePath,
    pub metadata: Option<PitouFileMetadata>,
}

impl PitouFile {
    pub fn clone_inner(self: &Rc<PitouFile>) -> Self {
        let path = self.path.path.clone();
        Self::without_metadata(PitouFilePath::from_pathbuf(path))
    }

    pub fn without_metadata(path: PitouFilePath) -> Self {
        Self {
            path,
            metadata: None,
        }
    }

    pub fn kind(&self) -> Option<PitouFileKind> {
        self.metadata().as_ref().map(|v| v.kind)
    }

    pub fn full_path_str(&self) -> &str {
        self.path.path.to_str().unwrap_or_default()
    }

    pub fn matches_find(self: &Rc<PitouFile>, key: &str) -> Option<Rc<PitouFile>> {
        if crate::extra::contains_ignore_case(key, self.name()) {
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn is_dir(&self) -> bool {
        match &self.metadata {
            None => false,
            Some(metadata) => matches!(metadata.kind, PitouFileKind::Directory),
        }
    }

    pub fn is_sys_item(&self) -> bool {
        match &self.metadata {
            None => false,
            Some(metadata) => metadata.is_sys_item(),
        }
    }

    pub fn is_link(&self) -> bool {
        match &self.metadata {
            None => false,
            Some(metadata) => matches!(metadata.kind, PitouFileKind::Link),
        }
    }
    pub fn is_file(&self) -> bool {
        match &self.metadata {
            None => false,
            Some(metadata) => matches!(metadata.kind, PitouFileKind::File),
        }
    }

    pub fn name(&self) -> &str {
        self.path.name()
    }

    pub fn name_without_extension(&self) -> &str {
        if self.is_dir() {
            self.name()
        } else {
            let name = self.name();
            let end = (0..name.len())
                .rev()
                .find(|&v| name.as_bytes()[v] == b'.')
                .unwrap_or(name.len());
            &name[0..end]
        }
    }

    pub fn path(&self) -> &PitouFilePath {
        &self.path
    }

    pub fn metadata(&self) -> &Option<PitouFileMetadata> {
        &self.metadata
    }
}

pub struct PitouTrashItem {
    pub original_path: PitouFilePath,
    pub metadata: PitouTrashItemMetadata,
}

impl PitouTrashItem {
    pub fn path(&self) -> &PitouFilePath {
        &self.original_path
    }

    pub fn id(&self) -> &str {
        &self.metadata().id
    }

    pub fn metadata(&self) -> &PitouTrashItemMetadata {
        &self.metadata
    }
    pub fn name(&self) -> &str {
        self.original_path.name()
    }
    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir
    }
}

#[derive(Serialize, Deserialize)]
pub struct PitouTrashItemMetadata {
    pub id: String,
    pub deleted: PitouDateTime,
    pub size: PitouFileSize,
    pub is_dir: bool,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PitouFileSortOrder {
    Increasing,
    Decreasing,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PitouFileSort {
    DateCreated(PitouFileSortOrder),
    DateModified(PitouFileSortOrder),
    DateAccessed(PitouFileSortOrder),
    Name(PitouFileSortOrder),
}

impl PitouFileSort {
    pub fn sorted(self, mut items: Vec<PitouFile>) -> Vec<PitouFile> {
        match self {
            PitouFileSort::DateCreated(order) => match order {
                PitouFileSortOrder::Increasing => {
                    items.sort_unstable_by_key(|v| v.metadata.as_ref().map(|m| m.created.datetime))
                }
                PitouFileSortOrder::Decreasing => items.sort_unstable_by_key(|v| {
                    v.metadata.as_ref().map(|m| Reverse(m.created.datetime))
                }),
            },
            PitouFileSort::Name(order) => match order {
                PitouFileSortOrder::Increasing => {
                    items.sort_unstable_by(|a, b| a.name().cmp(&b.name()))
                }
                PitouFileSortOrder::Decreasing => {
                    items.sort_unstable_by(|a, b| b.name().cmp(&a.name()))
                }
            },
            PitouFileSort::DateModified(order) => match order {
                PitouFileSortOrder::Increasing => {
                    items.sort_unstable_by_key(|v| v.metadata.as_ref().map(|m| m.modified.datetime))
                }
                PitouFileSortOrder::Decreasing => items.sort_unstable_by_key(|v| {
                    v.metadata.as_ref().map(|m| Reverse(m.modified.datetime))
                }),
            },
            PitouFileSort::DateAccessed(order) => match order {
                PitouFileSortOrder::Increasing => {
                    items.sort_unstable_by_key(|v| v.metadata.as_ref().map(|m| m.accessed.datetime))
                }
                PitouFileSortOrder::Decreasing => items.sort_unstable_by_key(|v| {
                    v.metadata.as_ref().map(|m| Reverse(m.accessed.datetime))
                }),
            },
        }
        items
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct PitouFileFilter {
    pub files: bool,
    pub links: bool,
    pub dirs: bool,
    pub sys_items: bool,
}

impl PitouFileFilter {
    pub fn new() -> Self {
        Self {
            files: true,
            links: true,
            dirs: true,
            sys_items: false,
        }
    }

    pub fn only_dirs() -> Self {
        Self {
            files: false,
            links: false,
            dirs: true,
            sys_items: false,
        }
    }

    pub fn map(self, file: PitouFile) -> Option<PitouFile> {
        if (file.is_dir() && self.dirs)
            || (file.is_file() && self.files)
            || (file.is_link() && self.links)
            || (file.is_sys_item() && self.sys_items)
        {
            Some(file)
        } else {
            None
        }
    }

    pub fn all_filtered(self) -> bool {
        !self.dirs && !self.files && !self.links
    }
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f, "rgba({}, {}, {}, {})", self.0, self.1, self.2, self.3}
    }
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct ColorTheme {
    pub background1: Color,
    pub background2: Color,
    pub foreground1: Color,
    pub foreground2: Color,
    pub spare1: Color,
    pub spare2: Color,
}

impl ColorTheme {
    pub const DEFAULT_DARK: Self = Self {
        background1: Color(60, 60, 60, 255),
        background2: Color(105, 105, 105, 255),
        foreground1: Color(240, 240, 240, 255),
        foreground2: Color(100, 200, 150, 255),
        spare1: Color(40, 40, 40, 255),
        spare2: Color(185, 210, 235, 255),
    };

    pub const DEFAULT_LIGHT: Self = Self {
        background1: Color(230, 230, 230, 255),
        background2: Color(180, 180, 180, 255),
        foreground1: Color(50, 50, 50, 255),
        foreground2: Color(80, 80, 80, 255),
        spare1: Color(80, 80, 80, 255),
        spare2: Color(30, 120, 50, 255),
    };

    pub const GEM_DARK: Self = Self {
        background1: Color(50, 50, 50, 255),
        background2: Color(30, 30, 30, 255),
        foreground1: Color(240, 240, 240, 255),
        foreground2: Color(0, 255, 255, 255),
        spare1: Color(100, 100, 100, 255),
        spare2: Color(255, 192, 203, 255),
    };

    pub const POLISH_DARK: Self = Self {
        background1: Color(30, 30, 30, 255),
        background2: Color(60, 60, 60, 255),
        foreground1: Color(220, 220, 220, 255),
        foreground2: Color(50, 150, 50, 255),
        spare1: Color(10, 10, 10, 255),
        spare2: Color(120, 180, 240, 255),
    };
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AppMenu {
    Home,
    Explorer,
    Trash,
    Favorites,
    Search,
    Locked,
    Recents,
    Cloud,
    Settings,
}

#[derive(PartialEq)]
pub enum GeneralFolder {
    DocumentsFolder(PitouFilePath),
    AudiosFolder(PitouFilePath),
    PicturesFolder(PitouFilePath),
    VideosFolder(PitouFilePath),
    DesktopFolder(PitouFilePath),
    DownloadsFolder(PitouFilePath),
}

impl GeneralFolder {
    pub fn o_name(&self) -> &str {
        match self {
            GeneralFolder::DocumentsFolder(d) => d.name(),
            GeneralFolder::AudiosFolder(a) => a.name(),
            GeneralFolder::PicturesFolder(p) => p.name(),
            GeneralFolder::VideosFolder(v) => v.name(),
            GeneralFolder::DesktopFolder(d) => d.name(),
            GeneralFolder::DownloadsFolder(d) => d.name(),
        }
    }

    pub fn as_pitou_file(&self) -> PitouFile {
        let path = self.path().path.clone().into();
        PitouFile::without_metadata(path)
    }

    pub fn name(&self) -> String {
        match self {
            GeneralFolder::DocumentsFolder(_) => String::from("Documents"),
            GeneralFolder::AudiosFolder(_) => String::from("Audios"),
            GeneralFolder::PicturesFolder(_) => String::from("Pictures"),
            GeneralFolder::VideosFolder(_) => String::from("Videos"),
            GeneralFolder::DesktopFolder(_) => String::from("Desktop"),
            GeneralFolder::DownloadsFolder(_) => String::from("Downloads"),
        }
    }

    pub fn path(&self) -> &PitouFilePath {
        match self {
            GeneralFolder::DocumentsFolder(path) => path,
            GeneralFolder::AudiosFolder(path) => path,
            GeneralFolder::PicturesFolder(path) => path,
            GeneralFolder::VideosFolder(path) => path,
            GeneralFolder::DesktopFolder(path) => path,
            GeneralFolder::DownloadsFolder(path) => path,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ItemsView {
    Grid,
    Rows,
    Tiles,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub refresh_rate: u8, // a number in the range 1..=60 (15000)
    pub show_extensions: bool,
    pub hide_system_files: bool,
    pub show_thumbnails: bool,
    pub items_view: ItemsView,
    pub show_parents: bool,
    pub items_sort: Option<PitouFileSort>,
    pub items_zoom: f32,
}

impl AppSettings {
    pub fn default_refresh_rate() -> u8 {
        3
    }

    pub fn refresh_rate_as_millis(&self) -> u32 {
        /*
            60 => 250 millis
            1 => 15000 millis
        */
        15000 / self.refresh_rate as u32
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            refresh_rate: Self::default_refresh_rate(),
            show_extensions: true,
            hide_system_files: true,
            show_thumbnails: false,
            items_view: ItemsView::Rows,
            show_parents: false,
            items_zoom: 1.0,
            items_sort: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DirChild {
    name: String,
    metadata: Option<PitouFileMetadata>,
}

#[derive(PartialEq, Clone)]
pub struct FrontendSearchOptions {
    pub input: String,
    pub search_kind: u8,
    pub depth: u8,
    pub case_sensitive: bool,
    pub hardware_accelerate: bool,
    pub skip_errors: bool,
    pub filter: PitouFileFilter,
    pub max_finds: usize,
}
