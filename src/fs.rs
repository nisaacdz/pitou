use super::size;
use std::{fs, io, path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Read,
    Write,
}

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    File,
    Folder,
}

pub struct Metadata {
    filetype: FileType,
    size: u64,
    permission: Permission,
}

impl Metadata {
    pub fn of(path: &path::PathBuf) -> Result<Metadata, io::Error> {
        let metadata = path::Path::metadata(path)?;
        let filetype = if metadata.is_dir() {
            FileType::Folder
        } else {
            FileType::File
        };
        let size = metadata.len();
        let permission = if metadata.permissions()./*TODO*/readonly() {
            Permission::Read
        } else {
            Permission::Write
        };

        Ok(Self {
            filetype,
            size,
            permission,
        })
    }

    pub fn permission(&self) -> Permission {
        self.permission
    }

    pub fn size<S: size::Unit>(&self) -> S {
        S::from_bytes(self.size)
    }

    pub fn filetype(&self) -> FileType {
        self.filetype
    }
}

pub enum DirContent {
    File(File),
    Error(io::Error),
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

pub struct File {
    path: path::PathBuf,
    metadata: Metadata,
}

impl File {
    pub fn create(path: path::PathBuf) -> Result<File, io::Error> {
        fs::File::create(&path)?;
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
    }

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

    pub fn open(&self) -> Result<fs::File, io::Error> {
        fs::File::open(self.path())
    }

    pub fn path(&self) -> &path::PathBuf {
        &self.path
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    // Returns the files within the current directory
    pub fn children(&self) -> Result<Vec<DirContent>, io::Error> {
        Self::files_in(self.path())
    }
}
