use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{path::PathBuf, rc::Rc};

use crate::{
    msg::SearchMsg, search::SimplifiedSearchOptions, GeneralFolder, PitouDrive, PitouDriveKind,
    PitouFile, PitouFileFilter, PitouFileMetadata, PitouFilePath, PitouTrashItem,
    PitouTrashItemMetadata,
};

const BMS: u8 = b'\\';
const FMS: u8 = b'/';

impl<'d> Deserialize<'d> for PitouFile {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        let path = PitouFilePath::deserialize(dz)?;
        let res = PitouFile {
            path,
            metadata: None,
        };
        Ok(res)
    }
}

impl Serialize for PitouFile {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct PitouFile<'a> {
            path: &'a PitouFilePath,
            metadata: &'a Option<PitouFileMetadata>,
        }

        let items = PitouFile {
            path: &self.path,
            metadata: &self.metadata,
        }
        .serialize(sz)?;
        Ok(items)
    }
}

impl Serialize for PitouDrive {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct PitouDrive<'a> {
            name: &'a String,
            mount_point: &'a PitouFilePath,
            total_space: u64,
            free_space: u64,
            is_removable: bool,
            kind: PitouDriveKind,
        }

        PitouDrive {
            name: &self.name,
            mount_point: &self.mount_point,
            total_space: self.total_space,
            free_space: self.free_space,
            is_removable: self.is_removable,
            kind: self.kind,
        }
        .serialize(sz)
    }
}

impl<'d> Deserialize<'d> for PitouFilePath {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        let path = deserialize_pathbuf(dz)?;
        Ok(path.into())
    }
}

impl Serialize for PitouFilePath {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        serialize_pathbuf(&self.path, sz)
    }
}

fn parse_path(mut path_str: String) -> PathBuf {
    for bc in unsafe { path_str.as_bytes_mut() } {
        if *bc == FMS {
            *bc = BMS;
        }
    }
    if path_str.len() > 0 {
        if path_str.len() == 1 {
            path_str.push(':');
            path_str.push(BMS as char);
        } else if path_str.len() == 2 && path_str.as_bytes()[1] == b':' {
            path_str.push(BMS as char);
        } else if path_str.as_bytes()[1] == BMS as u8 {
            path_str.insert(1, ':');
        }
    }
    PathBuf::from(path_str)
}

#[inline]
fn serialize_pathbuf<S: Serializer>(path: &PathBuf, sz: S) -> Result<S::Ok, S::Error> {
    let path = path.as_os_str().to_str().unwrap_or_default();
    sz.serialize_str(path)
}

#[inline]
fn deserialize_pathbuf<'d, D: Deserializer<'d>>(dz: D) -> Result<PathBuf, D::Error> {
    let dz = String::deserialize(dz)?;
    Ok(parse_path(dz))
}

mod deserialize_rc_pitoufile {
    #![allow(unused)]
    use super::*;
    pub fn deserialize<'d, D: Deserializer<'d>>(dz: D) -> Result<Rc<PitouFile>, D::Error> {
        let path = PitouFilePath::deserialize(dz)?;
        let item = PitouFile {
            path,
            metadata: None,
        };
        Ok(Rc::new(item))
    }
}

impl<'d> Deserialize<'d> for SimplifiedSearchOptions {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct SimplifiedSearchOptions {
            search_dir: PitouFile,
            hardware_accelerate: bool,
            filter: PitouFileFilter,
            case_sensitive: bool,
            depth: u8,
            input: String,
            search_kind: u8,
            skip_errors: bool,
            max_finds: usize,
        }
        let SimplifiedSearchOptions {
            search_dir,
            hardware_accelerate,
            filter,
            max_finds,
            depth,
            skip_errors,
            input,
            search_kind,
            case_sensitive,
        } = SimplifiedSearchOptions::deserialize(dz)?;
        let res = Self {
            search_dir,
            hardware_accelerate,
            filter,
            case_sensitive,
            depth,
            input,
            search_kind,
            skip_errors,
            max_finds,
        };
        Ok(res)
    }
}

impl Serialize for PitouTrashItem {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct PitouTrashItem<'a> {
            original_path: &'a PitouFilePath,
            metadata: &'a PitouTrashItemMetadata,
        }

        PitouTrashItem {
            original_path: &self.original_path,
            metadata: &self.metadata,
        }
        .serialize(sz)
    }
}

impl Serialize for GeneralFolder {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        enum GeneralFolder<'a> {
            DocumentsFolder(&'a PitouFilePath),
            AudiosFolder(&'a PitouFilePath),
            PicturesFolder(&'a PitouFilePath),
            VideosFolder(&'a PitouFilePath),
            DesktopFolder(&'a PitouFilePath),
            DownloadsFolder(&'a PitouFilePath),
        }

        match self {
            Self::DocumentsFolder(path) => GeneralFolder::DocumentsFolder(path),
            Self::AudiosFolder(path) => GeneralFolder::AudiosFolder(path),
            Self::PicturesFolder(path) => GeneralFolder::PicturesFolder(path),
            Self::VideosFolder(path) => GeneralFolder::VideosFolder(path),
            Self::DesktopFolder(path) => GeneralFolder::DesktopFolder(path),
            Self::DownloadsFolder(path) => GeneralFolder::DownloadsFolder(path),
        }
        .serialize(sz)
    }
}

impl Serialize for SearchMsg {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        use std::collections::LinkedList;
        #[derive(Serialize)]
        enum SearchMsg<'a> {
            Active(&'a LinkedList<PitouFile>),
            Terminated(&'a LinkedList<PitouFile>),
        }
        let fake_msg = match self {
            Self::Active(ll) => SearchMsg::Active(ll),
            Self::Terminated(ll) => SearchMsg::Terminated(ll),
        };

        fake_msg.serialize(sz)
    }
}
