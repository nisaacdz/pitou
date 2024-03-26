use std::{marker::PhantomData, rc::Rc};

use pitou_core::*;
use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize)]
pub struct NoArg;

#[derive(Serialize)]
pub struct DirChildrenArgs<'a> {
    pub dir: &'a PitouFilePath,
    pub filter: PitouFileFilter,
    pub sort: Option<PitouFileSort>,
}

impl<'a> DirChildrenArgs<'a> {
    pub fn new_default(dir: &'a PitouFilePath) -> Self {
        Self {
            dir,
            filter: PitouFileFilter::new(),
            sort: None,
        }
    }
}

pub struct GeneralFolderElems {
    pub items: Vec<Rc<GeneralFolder>>,
}

pub struct DriveItems {
    pub items: Rc<Vec<Rc<PitouDrive>>>,
}

impl Default for DriveItems {
    fn default() -> Self {
        Self {
            items: Rc::new(Vec::new()),
        }
    }
}

impl<'d> Deserialize<'d> for DriveItems {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        let items = rc_serde::deserialize(dz)?;
        Ok(Self {
            items: Rc::new(items),
        })
    }
}

impl<'d> Deserialize<'d> for GeneralFolderElems {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        struct VMS;
        impl<'d> Visitor<'d> for VMS {
            type Value = Vec<Rc<GeneralFolder>>;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "expecting a GeneralFolder instance")
            }
            fn visit_seq<A: SeqAccess<'d>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut items = Vec::new();
                while let Some(elem) = seq.next_element::<GeneralFolder>()? {
                    items.push(Rc::new(elem))
                }
                Ok(items)
            }
        }
        let items = dz.deserialize_seq(VMS)?;
        Ok(Self { items })
    }
}

mod rc_serde {
    use super::*;
    pub fn deserialize<'d, D: Deserializer<'d>, T: Deserialize<'d>>(
        dz: D,
    ) -> Result<Vec<Rc<T>>, D::Error> {
        struct VMS<T> {
            data: PhantomData<T>,
        }

        impl<'d, T: Deserialize<'d>> Visitor<'d> for VMS<T> {
            type Value = Vec<Rc<T>>;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "expecting a GeneralFolder instance")
            }
            fn visit_seq<A: SeqAccess<'d>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut items = Vec::new();
                while let Some(elem) = seq.next_element::<T>()? {
                    items.push(Rc::new(elem))
                }
                Ok(items)
            }
        }
        let items = dz.deserialize_seq(VMS {
            data: PhantomData::<T>,
        })?;
        Ok(items)
    }
}
