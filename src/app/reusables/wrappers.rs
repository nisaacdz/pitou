use std::rc::Rc;

use pitou_core::PitouTrashItem;

#[derive(Clone)]
pub struct TrashItem {
    item: Rc<PitouTrashItem>,
}

impl TrashItem {
    pub fn new(item: Rc<PitouTrashItem>) -> Self {
        Self { item }
    }
}

impl PartialEq for TrashItem {
    fn eq(&self, other: &Self) -> bool {
        self.item.path() == other.item.path()
    }
}

impl Eq for TrashItem {

}

impl std::hash::Hash for TrashItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.item.path().as_bytes());
    }
}