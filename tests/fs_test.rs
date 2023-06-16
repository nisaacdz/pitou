mod tests {
    use pitu::fs::*;
    use std::{fs, path};

    #[test]
    fn test_file_creation() {
        let file_path = path::PathBuf::from("target/tmp/test_file_creation.txt");

        // Clean up the file if it already exists
        let file = if !file_path.exists() {
            File::create(file_path.clone()).unwrap()
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

        // Clean up the file
        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_file_children() {
        let dir_path = path::PathBuf::from("target/tmp/test_file_children");

        // Create a directory
        if !dir_path.exists() {
            fs::create_dir(&dir_path).unwrap();
        }

        // Create some files inside the directory
        fs::write(dir_path.join("file1.txt"), "File 1").unwrap();
        fs::write(dir_path.join("file2.txt"), "File 2").unwrap();
        fs::create_dir(dir_path.join("subdir")).unwrap();

        let file = File::get(dir_path.clone()).unwrap();

        // Get the children of the directory
        let children = file.children().unwrap();

        // Check if the number of children is correct
        assert_eq!(children.len(), 3);

        // Check if the children are files or errors
        assert!(matches!(children[0], DirContent::File(_)));
        assert!(matches!(children[1], DirContent::File(_)));
        assert!(matches!(children[2], DirContent::File(_)));

        // Clean up the directory
        fs::remove_dir_all(&dir_path).unwrap();
    }
}
