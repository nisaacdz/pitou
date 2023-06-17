//! # pitou
//! **pitou** is a file system implementation that is independent of the underlining operating system.
//!
//! It provides high level features like **Metadata** and **PathBuf**.


mod fs;

pub use fs::*;

pub mod units;