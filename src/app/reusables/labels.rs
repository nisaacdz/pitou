use std::rc::Rc;

use pitou_core::{
    frontend::{GeneralFolder, PitouFileFilter, PitouFileSort},
    PitouFile, PitouFilePath,
};
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

pub struct DirChildren {
    pub children: Vec<Rc<PitouFile>>,
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

pub struct GeneralFolderElems {
    pub items: Vec<Rc<GeneralFolder>>,
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
