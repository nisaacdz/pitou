use std::rc::Rc;

use crate::{FrontendSearchOptions, PitouFile, PitouFileFilter};

pub struct SimplifiedSearchOptions {
    pub search_dir: PitouFile,
    pub input: String,
    pub search_kind: u8,
    pub depth: u8,
    pub case_sensitive: bool,
    pub hardware_accelerate: bool,
    pub skip_errors: bool,
    pub filter: PitouFileFilter,
    pub max_finds: usize,
}

impl SimplifiedSearchOptions {
    pub fn build_from(options: FrontendSearchOptions, current_dir: Rc<PitouFile>) -> Self {
        Self {
            search_dir: current_dir.clone_inner(),
            input: options.input,
            search_kind: options.search_kind,
            depth: options.depth,
            case_sensitive: options.case_sensitive,
            hardware_accelerate: options.hardware_accelerate,
            skip_errors: options.skip_errors,
            filter: options.filter,
            max_finds: options.max_finds,
        }
    }
}
