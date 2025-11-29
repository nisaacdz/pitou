use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::PitouFileSize;

/* *
impl Serialize for TabCtx {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        let mut ss = sz.serialize_struct("TabCtx", 6)?;
        if let Some(dir) = &*self.current_dir.borrow() {
            ss.serialize_field("current_dir", &dir.path)?;
        } else {
            ss.serialize_field("current_dir", &None::<Option<PitouFile>>)?;
        }
        ss.serialize_field("current_menu", &self.current_menu)?;
        ss.serialize_field("selected_files", &Vec::<PitouFile>::new())?;
        ss.serialize_field("search_results", &None::<Option<Vec<PitouFile>>>)?;
        ss.serialize_field("dir_children", &None::<Option<Vec<PitouFile>>>)?;
        ss.serialize_field("dir_siblings", &None::<Option<Vec<PitouFile>>>)?;
        ss.end()
    }
}

impl<'d> Deserialize<'d> for TabCtx {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        #[derive(Serialize, Deserialize)]
        struct TempVal {
            pub current_dir: PitouFilePath,
            pub current_menu: AppMenu,
            pub search_results: Option<Vec<PitouFile>>,
            pub dir_children: Option<Vec<PitouFile>>,
            pub dir_siblings: Option<Vec<PitouFile>>,
        }

        let TempVal {
            current_dir,
            current_menu,
            search_results: _,
            dir_children: _,
            dir_siblings: _,
        } = TempVal::deserialize(dz)?;

        let current_dir = Rc::new(PitouFile {
            path: current_dir,
            metadata: None,
        });
        Ok(TabCtx::new_with_dir(current_dir, current_menu))
    }
}

*/

impl Serialize for PitouFileSize {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        sz.serialize_u64(self.bytes)
    }
}

impl<'d> Deserialize<'d> for PitouFileSize {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        let bytes = u64::deserialize(dz)?;
        Ok(Self { bytes })
    }
}
