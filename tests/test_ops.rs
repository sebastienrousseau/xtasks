#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use tempfile::tempdir;
    use xtasks::ops::{clean_files, remove_dir, remove_file};

    #[test]
    fn test_remove_dir_not_empty() {
        let tmp_dir = tempdir().unwrap();
        let sub_dir = tmp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        File::create(sub_dir.join("file.txt")).unwrap();

        // This should succeed as `remove_dir` now removes non-empty directories.
        assert!(remove_dir(&sub_dir).is_ok());
        assert!(!sub_dir.exists());
    }

    #[test]
    fn test_clean_files_directory() {
        let tmp_dir = tempdir().unwrap();
        let sub_dir = tmp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        // clean_files should remove the directory
        assert!(clean_files(sub_dir.to_str().unwrap()).is_ok());
        assert!(!sub_dir.exists());
    }

    #[test]
    fn test_clean_files_match_directories() {
        let tmp_dir = tempdir().unwrap();
        let sub_dir = tmp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        // clean_files should remove the directory
        assert!(clean_files(
            tmp_dir.path().join("*").to_str().unwrap()
        )
        .is_ok());
        assert!(!sub_dir.exists());
    }

    #[test]
    fn test_remove_dir_non_existent() {
        let tmp_dir = tempdir().unwrap();
        let non_existent_dir = tmp_dir.path().join("nonexistent");
        assert!(remove_dir(non_existent_dir).is_err());
    }

    #[test]
    fn test_remove_file() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("testfile.txt");
        File::create(&file_path).unwrap();

        assert!(remove_file(&file_path).is_ok());
        assert!(!file_path.exists());
    }

    #[test]
    fn test_clean_files() {
        let tmp_dir = tempdir().unwrap();
        let file1 = tmp_dir.path().join("file1.txt");
        let file2 = tmp_dir.path().join("file2.txt");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        clean_files(tmp_dir.path().join("*.txt").to_str().unwrap())
            .unwrap();
        assert!(!file1.exists());
        assert!(!file2.exists());
    }

    #[test]
    fn test_clean_files_no_match() {
        let tmp_dir = tempdir().unwrap();
        let file1 = tmp_dir.path().join("file1.txt");
        File::create(&file1).unwrap();

        clean_files(tmp_dir.path().join("*.log").to_str().unwrap())
            .unwrap();
        assert!(file1.exists());
    }

    #[test]
    fn test_remove_file_directory() {
        let tmp_dir = tempdir().unwrap();
        let sub_dir = tmp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        // remove_file should fail on a directory
        assert!(remove_file(&sub_dir).is_err());
    }

    #[test]
    fn test_remove_file_non_existent() {
        let tmp_dir = tempdir().unwrap();
        let non_existent_file = tmp_dir.path().join("nonexistent.txt");
        assert!(remove_file(non_existent_file).is_err());
    }
}
