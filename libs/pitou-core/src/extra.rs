use crate::{PitouFile, PitouFilePath, PitouFileSize};
use std::{hash::Hash, path::PathBuf};

impl From<u64> for PitouFileSize {
    fn from(bytes: u64) -> Self {
        Self { bytes }
    }
}

impl From<PathBuf> for PitouFilePath {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Hash for PitouFilePath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.path.as_os_str().as_encoded_bytes())
    }
}

impl PartialEq for PitouFile {
    fn eq(&self, other: &Self) -> bool {
        &self.path == &other.path
    }
}

impl AsRef<std::path::Path> for PitouFile {
    fn as_ref(&self) -> &std::path::Path {
        &self.path.path
    }
}

pub fn starts_with_ignore_case(key: &str, input: &str) -> bool {
    if input.len() < key.len() {
        return false;
    }
    input
        .chars()
        .take(key.len())
        .zip(key.chars())
        .all(|(w, k)| w.eq_ignore_ascii_case(&k))
}

pub fn ends_with_ignore_case(key: &str, input: &str) -> bool {
    if input.len() < key.len() {
        return false;
    }
    input
        .chars()
        .rev()
        .take(key.len())
        .zip(key.chars().rev())
        .all(|(a, b)| a.eq_ignore_ascii_case(&b))
}

pub fn contains_ignore_case(key: &str, input: &str) -> bool {
    let key = key.as_bytes();
    let input = input.as_bytes();
    if input.len() < key.len() {
        return false;
    }
    input.windows(key.len()).any(|window| {
        (0..window.len()).all(|idx| (key[idx] as char).eq_ignore_ascii_case(&(window[idx] as char)))
    })
}
