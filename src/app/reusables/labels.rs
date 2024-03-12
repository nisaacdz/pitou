use std::rc::Rc;

use pitou_core::frontend::GeneralFolder;
use serde::{de::{SeqAccess, Visitor}, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NoArg;

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