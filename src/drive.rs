use std::{ffi::OsString, fs, io, path};
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};

use crate::units::*;

use super::{DirContent, DirEntryResult, FilesIn};

#[derive(Debug, Clone)]
pub struct Drive {
    label: std::ffi::OsString,
    path: path::PathBuf,
    space: u64,
    free: u64,
}

impl Drive {
    pub fn total_space<S: Unit>(&self) -> S {
        S::from_bytes(self.space)
    }

    pub fn free_space<S: Unit>(&self) -> S {
        S::from_bytes(self.free)
    }

    pub fn used_space<S: Unit>(&self) -> S {
        S::from_bytes(self.space - self.free)
    }

    pub fn label(&self) -> &OsString {
        &self.label
    }

    pub fn path(&self) -> &path::PathBuf {
        &self.path
    }
}

macro_rules! drive {
    ($drive:expr) => {
        Drive {
            label: $drive.name().to_os_string(),
            path: $drive.mount_point().into(),
            space: $drive.total_space(),
            free: $drive.available_space(),
        }
    };
}

impl Drive {
    pub fn entries(&self) -> io::Result<FilesIn<impl FnMut(DirEntryResult) -> DirContent>> {
        let res = fs::read_dir(&self.path)?.map(|e| e.into());
        Ok(FilesIn::new(res))
    }
}

pub fn available() -> Vec<Drive> {
    System::new_with_specifics(RefreshKind::new().with_disks_list())
        .disks()
        .iter()
        .map(|d| drive!(d))
        .collect()
    //sysinfo::System::new_all().disks().iter().map(|d| drive!(d)).collect()
}

#[test]
fn test_path() {
    let buf = path::PathBuf::from("");
    let d = buf.display();
}

#[test]
fn test_available() {
    for drive in available() {
        println!(
            "name: {:?}, path: {:?}, space: {:?}, free: {:?}",
            drive.label().to_str().unwrap(),
            drive.path(),
            drive.total_space::<Gigabytes>(),
            drive.free_space::<Gigabytes>()
        );

        for entry in drive.entries().unwrap() {
            match entry {
                DirContent::File(f) => println!("{:?}", f),
                DirContent::Error(_) => println!("Something occured"),
            }
        }
    }
}
