#[cfg(test)]
mod tests {

    use xtasks::ops::*;
    use tempfile::tempdir;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_clean_files() {
        let tmp_dir = tempdir().unwrap();
        fs::File::create(tmp_dir.path().join("tmp1.txt")).unwrap();
        fs::File::create(tmp_dir.path().join("tmp2.txt")).unwrap();

        assert!(clean_files(tmp_dir.path().join("*.txt").to_str().unwrap()).is_ok());
        assert!(!exists(tmp_dir.path().join("tmp1.txt")));
        assert!(!exists(tmp_dir.path().join("tmp2.txt")));
    }

    #[test]
    fn test_remove_file() {
        let tmp_dir = tempdir().unwrap();
        let tmp_file = tmp_dir.path().join("tmp.txt");
        fs::File::create(&tmp_file).unwrap();

        assert!(remove_file(&tmp_file).is_ok());
        assert!(!exists(tmp_file));
    }

    #[test]
    fn test_remove_dir() {
        let tmp_dir = tempdir().unwrap();

        assert!(remove_dir(tmp_dir.path()).is_ok());
        assert!(!exists(tmp_dir.path()));
    }

    #[test]
    fn test_clean_files_directory() {
        let tmp_dir = tempdir().unwrap();
        let sub_dir = tmp_dir.path().join("sub_dir");
        fs::create_dir(&sub_dir).unwrap();
        fs::File::create(sub_dir.join("tmp1.txt")).unwrap();
        fs::File::create(sub_dir.join("tmp2.txt")).unwrap();

        // Since clean_files expects a file glob pattern, providing a directory path should result in an error.
        assert!(clean_files(sub_dir.to_str().unwrap()).is_err());
        assert!(exists(&sub_dir));
        assert!(exists(sub_dir.join("tmp1.txt")));
        assert!(exists(sub_dir.join("tmp2.txt")));
    }

    #[test]
    fn test_remove_file_non_existent() {
        assert!(remove_file(Path::new("non_existent_file.txt")).is_err());
    }

    #[test]
    fn test_remove_file_directory() {
        let tmp_dir = tempdir().unwrap();
        assert!(remove_file(tmp_dir.path()).is_err());
    }

    #[test]
    fn test_remove_dir_non_existent() {
        assert!(remove_dir(Path::new("non_existent_directory")).is_err());
    }

    #[test]
    fn test_remove_dir_file() {
        let tmp_dir = tempdir().unwrap();
        let tmp_file = tmp_dir.path().join("tmp.txt");
        fs::File::create(&tmp_file).unwrap();
        assert!(remove_dir(&tmp_file).is_err());
    }

    #[test]
    fn test_remove_dir_not_empty() {
        let tmp_dir = tempdir().unwrap();
        let sub_dir = tmp_dir.path().join("sub_dir");
        fs::create_dir(&sub_dir).unwrap();
        fs::File::create(sub_dir.join("tmp1.txt")).unwrap();
        assert!(remove_dir(&sub_dir).is_ok());
        assert!(!exists(&sub_dir));
    }

    #[test]
    fn test_clean_files_match_directories() {
        let tmp_dir = tempdir().unwrap();
        let sub_dir = tmp_dir.path().join("sub_dir");
        fs::create_dir(&sub_dir).unwrap();
        fs::File::create(sub_dir.join("tmp1.txt")).unwrap();
        fs::File::create(sub_dir.join("tmp2.txt")).unwrap();

        assert!(clean_files(tmp_dir.path().join("*").to_str().unwrap()).is_err());
        assert!(exists(&sub_dir));
        assert!(exists(sub_dir.join("tmp1.txt")));
        assert!(exists(sub_dir.join("tmp2.txt")));
    }

    #[test]
    fn test_clean_files_no_match() {
        let tmp_dir = tempdir().unwrap();
        fs::File::create(tmp_dir.path().join("tmp1.txt")).unwrap();
        fs::File::create(tmp_dir.path().join("tmp2.txt")).unwrap();

        assert!(clean_files(tmp_dir.path().join("*.md").to_str().unwrap()).is_ok());
        assert!(exists(tmp_dir.path().join("tmp1.txt")));
        assert!(exists(tmp_dir.path().join("tmp2.txt")));
    }

}