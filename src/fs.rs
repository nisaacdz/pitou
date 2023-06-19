use super::units;
use std::{
    fs, io, iter,
    path::{self, PathBuf},
};

type DirEntryResult = io::Result<fs::DirEntry>;

/// Represent if the described file has write access or not
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Read,
    Write,
}

/// Represents either a directory or a non-directory file.
#[derive(Debug, Clone, Copy)]
pub enum Category {
    File,
    Folder,
}

/// Some associated data stored alongside files.
pub struct Metadata {
    /// The category of the file, indicating whether it is a file or a folder.
    category: Category,
    /// The size of the file in bytes.
    size: u64,
    /// The permission associated with the file, indicating if it has read or write access.
    permission: Permission,
}

impl Metadata {
    /// Queries the file system for the metadata of the file at the given path
    pub fn of(path: &path::PathBuf) -> Result<Metadata, io::Error> {
        let metadata = path::Path::metadata(path)?;
        let category = if metadata.is_dir() {
            Category::Folder
        } else {
            Category::File
        };
        let size = metadata.len();
        let permission = if metadata.permissions().readonly() {
            Permission::Read
        } else {
            Permission::Write
        };

        Ok(Self {
            category,
            size,
            permission,
        })
    }

    // Returns the permission type contained in this `Metadata`
    pub fn permission(&self) -> Permission {
        self.permission
    }

    /// Returns the size of the file in the given unit.
    pub fn size<S: units::Unit>(&self) -> S {
        S::from_bytes(self.size)
    }

    /// Returns true if the file is a Directoroy
    pub fn category(&self) -> Category {
        self.category
    }
}

/// The contents of a directory at the time of reading.
///
/// Some entries may represent valid files (`DirContent::File(pitou::File)`),
/// while others may indicate errors or inaccessibility (`DirContent::Error(std::io::Error)`).
pub enum DirContent {
    /// Represents a valid file entry within a directory.
    File(File),
    /// Represents an error or inaccessibility encountered while reading a directory.
    Error(io::Error),
}

impl DirContent {
    /// confidently retrieves the file in the File variant of the DirContent assuming that self is indeed a file variant.
    pub fn unwrap(self) -> File {
        match self {
            Self::File(file) => file,
            Self::Error(e) => panic!("called unwrap on Error variant of DirContent. {}", e),
        }
    }
}

/// The contents of a file at the time of reading.
pub enum FileContent {
    /// Represents the actual file contents as a byte vector.
    File(Vec<u8>),
    /// Represents an error encountered while reading a file.
    Error(io::Error),
}

impl FileContent {
    /// Retrieves the File contents assuming the `FileContent` is of variant `FileContent::File(_)`
    pub fn unwrap(self) -> Vec<u8> {
        match self {
            FileContent::File(bytes) => bytes,
            FileContent::Error(e) => panic!("called unwrap in Error variant of FileContent. {}", e),
        }
    }
}

impl From<Result<File, io::Error>> for DirContent {
    fn from(value: Result<File, io::Error>) -> Self {
        match value {
            Ok(file) => file.into(),
            Err(error) => error.into(),
        }
    }
}

impl From<File> for DirContent {
    fn from(value: File) -> Self {
        Self::File(value)
    }
}

impl From<io::Error> for DirContent {
    fn from(value: io::Error) -> Self {
        Self::Error(value)
    }
}

/// Checks if the value is bigger than isize::MAX
macro_rules! overflows_isize {
    ($val:expr) => {
        ($val as i128) > (isize::MAX as i128)
    };
}

/// Represents a file in the file system.
///
/// A `File` may be a directory file or a non-directory file
pub struct File {
    /// The path of the file.
    path: path::PathBuf,
    /// The metadata associated with the file.
    metadata: Metadata,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
    }
}

impl From<DirEntryResult> for DirContent {
    fn from(val: DirEntryResult) -> Self {
        match val {
            Ok(v) => File::get(v.path()).into(),
            Err(e) => e.into(),
        }
    }
}

impl File {
    /// Creates a new file in the file system at the specified path and returns a `pitou::File` pointing to it.
    pub fn create_file(path: path::PathBuf) -> Result<File, io::Error> {
        fs::File::create(&path)?;
        let metadata = Metadata::of(&path)?;
        Ok(File { path, metadata })
    }

    /// Creates a new directory file in the file system at the specified path and returns a `pitou::File` pointing to it.
    pub fn create_dir(path: path::PathBuf) -> Result<File, io::Error> {
        fs::create_dir(&path)?;
        let metadata = Metadata::of(&path)?;
        Ok(File { path, metadata })
    }

    // Returns a pointer to an existing in the file system
    pub fn get(path: path::PathBuf) -> Result<File, io::Error> {
        let metadata = Metadata::of(&path)?;
        Ok(Self { path, metadata })
    }

    /// Updates the metadata of an existing `pitou::File` to match the described file
    pub fn refresh(&mut self) -> Result<(), io::Error> {
        let metadata = Metadata::of(self.path())?;
        self.metadata = metadata;
        Ok(())
    }

    /// Returns all the files contained inside the `dir_path` assuming that the dir_path is a valid directory path in the file system.
    ///
    /// Some valid paths may be in the directory but may be broken paths or may be deleted at time of checking
    ///
    /// For this reason, a `DirContent` which may or may not contain a valid `pitou::File` is returned.
    pub fn files_in(
        dir_path: &path::PathBuf,
    ) -> io::Result<FilesIn<impl FnMut(DirEntryResult) -> DirContent>> {
        let res = fs::read_dir(dir_path)?.map(|e| e.into());
        Ok(FilesIn(res))
    }

    /// Returns `true` if the current file is a directory file
    pub fn is_dir(&self) -> bool {
        matches!(self.metadata().category(), Category::Folder)
    }

    /// Attempts to open the pointed to file assuming it still exists or permissions still remain.
    ///
    /// This opens the file in read only mode.
    pub fn open(&self) -> Result<fs::File, io::Error> {
        fs::File::open(self.path())
    }
    /// Attempts to open the pointed to file assuming it still exists or permissions still remain.
    ///
    /// This opens the file in write only mode.
    pub fn open_mut(&mut self) -> Result<fs::File, io::Error> {
        fs::OpenOptions::new()
            .create(false)
            .append(true)
            .open(self.path())
    }

    /// Returns the path of this file
    pub fn path(&self) -> &path::PathBuf {
        &self.path
    }

    /// If the pointed to file is a normal file, the size is returned in bytes.
    ///
    /// If it is a directory, the bytes size of the entire contents of the directory is returned.
    pub fn size(&self) -> u64 {
        self.metadata().size
    }

    /// Returns a reference to the Metadata of the file.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns the content of the file in bytes
    pub fn content(&self) -> Result<FileContent, io::Error> {
        let mut file = self.open()?;
        let cpcs = self.metadata.size;
        if overflows_isize!(cpcs) {
            let res = io::Error::new(
                io::ErrorKind::OutOfMemory,
                "Cannot Read so many bytes at once",
            );
            Ok(FileContent::Error(res))
        } else {
            let mut res = vec![0; cpcs as usize];
            match io::Read::read_exact(&mut file, &mut res) {
                Ok(_) => Ok(FileContent::File(res)),
                Err(e) => Ok(FileContent::Error(e)),
            }
        }
    }

    // Returns the files contained within the current directory
    pub fn entries(&self) -> io::Result<FilesIn<impl FnMut(DirEntryResult) -> DirContent>> {
        Self::files_in(self.path())
    }
}

/// An iterator over the contents of a directory, producing `DirContent` items.
pub struct FilesIn<F>(iter::Map<fs::ReadDir, F>);

impl<F: FnMut(DirEntryResult) -> DirContent> Iterator for FilesIn<F> {
    type Item = DirContent;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl From<File> for Directory {
    fn from(file: File) -> Directory {
        Directory(file)
    }
}

/// Represents a directory in the file system.
pub struct Directory(File);

impl Directory {
    /// Create a specified directory or empty the directory if it already exist.
    pub fn create(path: PathBuf) -> Result<Self, io::Error> {
        let file = File::create_dir(path)?;
        Ok(Self(file))
    }

    /// Returns the contents of the directory as an iterator of File
    pub fn entries(&self) -> io::Result<FilesIn<impl FnMut(DirEntryResult) -> DirContent>> {
        self.0.entries()
    }

    // Returns the sum of all file sizes within this directory, recursively.
    /// Missen or inaccessible files return a size of 0bytes
    pub fn weight(&self) -> Result<u64, io::Error> {
        let sum = self.entries()?.map(|d| d.weight()).sum();
        Ok(sum)
    }
}

impl DirContent {
    /// Returns the sum of all file sizes within this directory, recursively.
    /// Returns 0 if the file doesn't not exist or if any other error is encountered
    fn weight(self) -> u64 {
        match self {
            DirContent::File(f) => {
                if f.is_dir() {
                    f.size()
                } else {
                    Directory(f).weight().unwrap_or(0)
                }
            }
            DirContent::Error(_) => 0,
        }
    }
}
