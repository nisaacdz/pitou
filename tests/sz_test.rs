#![cfg(test)]

use std::{io::Write, path};

use pitu::{fs::File, size::*};

#[test]
fn test_file_content_n_size() {
    let file_path = path::PathBuf::from("target/tmp/test_file_content_n_size.txt");

    // Create a file to test opening
    let mut pfile = if !file_path.exists() {
        File::create_file(file_path.clone()).unwrap()
    } else {
        File::get(file_path.clone()).unwrap()
    };

    let mut file = pfile.open_mut().unwrap();

    file.write_all(b"test_file_content_n_size").unwrap();

    pfile.refresh().unwrap();

    let b = pfile.metadata().size::<Kilobytes>();

    println!("Bytes {}\n Kilobytes {:?}", b.to_bytes(), b)
}

#[test]
fn test_size_conversion() {
    let file_path = path::PathBuf::from("target");

    let file = File::get(file_path.clone()).unwrap();

    let bytes = file.size().unwrap();

    println!("{:?}", Bytes::from_bytes(bytes));
    println!("{:?}", Kilobytes::from_bytes(bytes));
    println!("{:?}", Megabytes::from_bytes(bytes));
}
