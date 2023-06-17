use super::size;
use std::{
    fs, io,
    path::{self, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Read,
    Write,
}

/// Represents either a directory or a non-directory file
#[derive(Debug, Clone, Copy)]
pub enum Category {
    File,
    Folder,
}

pub struct Metadata {
    category: Category,
    size: u64,
    permission: Permission,
}

impl Metadata {
    /// Queries the system for the metadata of the file at the given path
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
    ///
    pub fn size<S: size::Unit>(&self) -> S {
        S::from_bytes(self.size)
    }

    /// Returns true if the file is a Directoroy
    pub fn category(&self) -> Category {
        self.category
    }
}

pub enum DirContent {
    File(File),
    Error(io::Error),
}

impl DirContent {
    pub fn unwrap(self) -> File {
        match self {
            Self::File(file) => file,
            Self::Error(e) => panic!("called unwrap on Error variant of DirContent. {}", e),
        }
    }
}

pub enum FileContent {
    File(Vec<u8>),
    Error(io::Error),
}

impl FileContent {
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

macro_rules! overflows_isize {
    ($val:expr) => {
        ($val as i128) > (isize::MAX as i128)
    };
}

pub struct File {
    path: path::PathBuf,
    metadata: Metadata,
}

impl File {
    pub fn create_file(path: path::PathBuf) -> Result<File, io::Error> {
        fs::File::create(&path)?;
        let metadata = Metadata::of(&path)?;
        Ok(File { path, metadata })
    }

    pub fn create_dir(path: path::PathBuf) -> Result<File, io::Error> {
        fs::create_dir(&path)?;
        let metadata = Metadata::of(&path)?;
        Ok(File { path, metadata })
    }

    pub fn get(path: path::PathBuf) -> Result<File, io::Error> {
        let metadata = Metadata::of(&path)?;
        Ok(Self { path, metadata })
    }

    pub fn refresh(&mut self) -> Result<(), io::Error> {
        let metadata = Metadata::of(self.path())?;
        self.metadata = metadata;
        Ok(())
    } //pitu

    pub fn files_in(dir_path: &path::PathBuf) -> Result<Vec<DirContent>, io::Error> {
        let mut res = Vec::new();
        for entry in fs::read_dir(dir_path)? {
            let file = match entry {
                Ok(cnt) => Self::get(cnt.path()).into(),
                Err(error) => error.into(),
            };
            res.push(file)
        }
        Ok(res)
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.metadata().category(), Category::Folder)
    }

    pub fn open(&self) -> Result<fs::File, io::Error> {
        fs::File::open(self.path())
    }

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

    /// Returns the size of the file or folder in bytes
    pub fn size(&self) -> Result<u64, io::Error> {
        let res = if self.is_dir() {
            let mut sz = 0;
            let cnts = self.entries()?;
            cnts.into_iter()
                .filter(|f| matches!(f, DirContent::File(_)))
                .map(|f| f.unwrap())
                .for_each(|file| {
                    sz += file.size().unwrap_or(0);
                });
            sz
        } else {
            self.metadata().size
        };

        Ok(res)
    }

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

    // Returns the files within the current directory
    pub fn entries(&self) -> Result<Vec<DirContent>, io::Error> {
        Self::files_in(self.path())
    }
}

pub struct Directory(File);

impl Directory {
    pub fn create(path: PathBuf) -> Result<Self, io::Error> {
        let file = File::create_dir(path)?;
        Ok(Self(file))
    }

    pub fn entries(&self) -> Result<Vec<DirContent>, io::Error> {
        self.0.entries()
    }
}
