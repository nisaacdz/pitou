#![cfg(test)]

use pitou::*;
use std::{fs, io, path};

#[test]
fn test_file_creation() {
    let file_path = path::PathBuf::from("target/tmp/test_file_creation.txt");

    // Clean up the file if it already exists
    let file = if !file_path.exists() {
        File::create_file(file_path.clone()).unwrap()
    } else {
        File::get(file_path.clone()).unwrap()
    };

    // Check if the file was created successfully
    assert_eq!(file.path(), &file_path);

    // Check if the file has write permission
    assert_eq!(file.metadata().permission(), Permission::Write);
}

#[test]
fn test_file_opening() {
    let file_path = path::PathBuf::from("target/tmp/test_file_opening.txt");

    // Create a file to test opening
    if !file_path.exists() {
        fs::File::create(&file_path).unwrap();
    }

    fs::write(&file_path, "Hello, World!").unwrap();

    let file = File::get(file_path.clone()).unwrap();

    // Check if the file was opened successfully
    assert_eq!(file.path(), &file_path);

    // Check if the file has read permission
    assert_eq!(file.metadata().permission(), Permission::Write);
}

#[test]
fn test_file_entries() {
    let dir_path = path::PathBuf::from("target/tmp/test_file_entries");

    // Create a directory
    if !dir_path.exists() {
        Directory::create(dir_path.clone()).unwrap();

        fs::write(dir_path.join("file1.txt"), "File 1").unwrap();
        fs::write(dir_path.join("file2.txt"), "File 2").unwrap();
        fs::create_dir(dir_path.join("subdir")).unwrap();
    }

    // Create some files inside the directory

    let file = File::get(dir_path.clone()).unwrap();

    println!("{:?}", file.path().to_str());

    // Get the contents of the directory
    let entries = file.entries().unwrap();

    // Check if the number of entries is correct
    assert_eq!(entries.len(), 3);

    // Check if the entries are files or errors
    assert!(matches!(entries[0], DirContent::File(_)));
    assert!(matches!(entries[1], DirContent::File(_)));
    assert!(matches!(entries[2], DirContent::File(_)));
}

#[test]
fn test_file_io() {
    use io::Write;
    let file_path = path::PathBuf::from("target/tmp/test_file_io.txt");

    let input = b"In the land of myth and in the time of magic";

    if !file_path.exists() {
        let mut file = File::create_file(file_path.clone()).unwrap();
        file.open_mut().unwrap().write(input).unwrap();
    }

    let file = File::get(file_path).unwrap();
    let output = file.content().unwrap().unwrap();

    assert_eq!(input.len(), output.len());

    assert_eq!(input.as_slice(), &output);

    println!("{}", String::from_utf8(output).unwrap());
}
