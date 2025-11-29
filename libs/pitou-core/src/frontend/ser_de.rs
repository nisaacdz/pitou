use std::{path::PathBuf, rc::Rc};

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{
    msg::SearchMsg, search::SimplifiedSearchOptions, GeneralFolder, PitouDrive, PitouDriveKind,
    PitouFile, PitouFileFilter, PitouFileMetadata, PitouFilePath, PitouTrashItem,
    PitouTrashItemMetadata,
};

use super::extra::DirChildren;

const BMS: u8 = b'\\';
const FMS: u8 = b'/';

#[inline]
fn serialize_pathbuf<S: Serializer>(path: &PathBuf, sz: S) -> Result<S::Ok, S::Error> {
    let path_str = path.as_os_str().to_str().unwrap_or_default();
    sz.serialize_str(path_str)
}

#[inline]
fn deserialize_pathbuf<'d, D: Deserializer<'d>>(dz: D) -> Result<PathBuf, D::Error> {
    let res = String::deserialize(dz)?;
    Ok(parse_path(res))
}

impl<'d> Deserialize<'d> for PitouFile {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct PitouFile {
            path: PitouFilePath,
            metadata: Option<PitouFileMetadata>,
        }

        let PitouFile { path, metadata } = PitouFile::deserialize(dz)?;
        Ok(Self { path, metadata })
    }
}

impl Serialize for PitouFile {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        self.path.serialize(sz)
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
        if *bc == BMS {
            *bc = FMS;
        }
    }
    PathBuf::from(path_str)
}

impl<'d> Deserialize<'d> for PitouDrive {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct PitouDrive {
            name: String,
            mount_point: PitouFilePath,
            total_space: u64,
            free_space: u64,
            is_removable: bool,
            kind: PitouDriveKind,
        }

        let PitouDrive {
            name,
            mount_point,
            total_space,
            free_space,
            is_removable,
            kind,
        } = PitouDrive::deserialize(dz)?;

        Ok(Self {
            name,
            mount_point,
            total_space,
            free_space,
            is_removable,
            kind,
        })
    }
}

impl<'d> Deserialize<'d> for DirChildren {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        struct VMS;
        impl<'v> Visitor<'v> for VMS {
            type Value = Vec<Rc<PitouFile>>;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "expecting a list of items")
            }
            fn visit_seq<A: SeqAccess<'v>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut res = Vec::new();
                while let Some(val) = seq.next_element()? {
                    res.push(Rc::new(val))
                }
                Ok(res)
            }
        }
        let children = dz.deserialize_seq(VMS)?;
        Ok(Self { children })
    }
}

mod serialize_rc_pitoufile {
    #![allow(unused)]
    use super::*;
    pub fn serialize<S: Serializer>(item: &Rc<PitouFile>, sz: S) -> Result<S::Ok, S::Error> {
        PitouFilePath::serialize(&item.path, sz)
    }
}
impl Serialize for SimplifiedSearchOptions {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct SimplifiedSearchOptions<'a> {
            search_dir: &'a PitouFile,
            hardware_accelerate: bool,
            filter: PitouFileFilter,
            case_sensitive: bool,
            depth: u8,
            input: &'a String,
            search_kind: u8,
            skip_errors: bool,
            max_finds: usize,
        }

        SimplifiedSearchOptions {
            search_dir: &self.search_dir,
            hardware_accelerate: self.hardware_accelerate,
            filter: self.filter,
            depth: self.depth,
            input: &self.input,
            search_kind: self.search_kind,
            skip_errors: self.skip_errors,
            max_finds: self.max_finds,
            case_sensitive: self.case_sensitive,
        }
        .serialize(sz)
    }
}

impl<'d> Deserialize<'d> for PitouTrashItem {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct PitouTrashItem {
            original_path: PitouFilePath,
            metadata: PitouTrashItemMetadata,
        }

        let PitouTrashItem {
            original_path,
            metadata,
        } = PitouTrashItem::deserialize(dz)?;
        Ok(Self {
            original_path,
            metadata,
        })
    }
}

impl<'d> Deserialize<'d> for GeneralFolder {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        enum GeneralFolder {
            DocumentsFolder(PitouFilePath),
            AudiosFolder(PitouFilePath),
            PicturesFolder(PitouFilePath),
            VideosFolder(PitouFilePath),
            DesktopFolder(PitouFilePath),
            DownloadsFolder(PitouFilePath),
        }

        let res = match GeneralFolder::deserialize(dz)? {
            GeneralFolder::DocumentsFolder(path) => Self::DocumentsFolder(path),
            GeneralFolder::AudiosFolder(path) => Self::AudiosFolder(path),
            GeneralFolder::PicturesFolder(path) => Self::PicturesFolder(path),
            GeneralFolder::VideosFolder(path) => Self::VideosFolder(path),
            GeneralFolder::DesktopFolder(path) => Self::DesktopFolder(path),
            GeneralFolder::DownloadsFolder(path) => Self::DownloadsFolder(path),
        };

        Ok(res)
    }
}

impl<'d> Deserialize<'d> for SearchMsg {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        use std::collections::LinkedList;
        #[derive(Deserialize)]
        enum SearchMsg {
            Active(LinkedList<PitouFile>),
            Terminated(LinkedList<PitouFile>),
        }
        let smg = SearchMsg::deserialize(dz)?;
        let real_msg = match smg {
            SearchMsg::Active(ll) => Self::Active(ll),
            SearchMsg::Terminated(ll) => Self::Terminated(ll),
        };
        Ok(real_msg)
    }
}
