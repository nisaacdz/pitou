use std::{rc::Rc, slice::Iter};

use crate::PitouFile;

pub struct FolderTracker {
    pub items: Vec<Rc<PitouFile>>,
    pub idx: usize,
}

impl FolderTracker {
    pub fn new(val: Rc<PitouFile>) -> Self {
        Self {
            items: vec![val],
            idx: 0,
        }
    }

    pub fn current(&self) -> Rc<PitouFile> {
        self.items[self.idx].clone()
    }

    pub fn all(&self) -> Iter<Rc<PitouFile>> {
        self.items.iter()
    }

    pub fn next(&self) -> Option<Rc<PitouFile>> {
        if self.idx + 1 >= self.items.len() {
            None
        } else {
            Some(self.items[self.idx + 1].clone())
        }
    }

    pub fn prev(&self) -> Option<Rc<PitouFile>> {
        if self.idx == 0 {
            None
        } else {
            Some(self.items[self.idx - 1].clone())
        }
    }

    pub fn update_directory(&mut self, new_dir: Rc<PitouFile>) {
        while self.idx + 1 < self.items.len() {
            self.items.pop();
        }
        self.items.push(new_dir);
        self.idx += 1;
    }

    pub fn go_backward(&mut self) {
        if self.idx > 0 {
            self.idx -= 1;
        }
    }

    pub fn go_forward(&mut self) {
        if self.idx + 1 < self.items.len() {
            self.idx += 1;
        }
    }
}

pub struct DirChildren {
    pub children: Vec<Rc<PitouFile>>,
}
