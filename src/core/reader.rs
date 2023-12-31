use std::fs::{File, self};
use std::io::{self, Read};
use crate::core::ignore::IgnorePattern;

pub fn read_workspace_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn read_workspace_dir(dir_path: &str) -> io::Result<Vec<u8>> {
    let metadata = fs::metadata(dir_path)?;
    if !metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a directory",
        ));
    }
    let mut dir_content = Vec::new();
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        dir_content.extend(path.to_str().unwrap().as_bytes());
        dir_content.extend("\n".as_bytes());
    }
    Ok(dir_content)
}

pub fn remove_end_of_line(buffer: &mut Vec<u8>) {
    if buffer.ends_with(&[b'\n']) {
        buffer.pop();
    }
    if buffer.ends_with(&[b'\r']) {
        buffer.pop();
    }
}

pub fn load_ignore_patterns() -> io::Result<Vec<IgnorePattern>> {
    let mut patterns = Vec::new();
    let file_content = fs::read_to_string(".rgitignore")?;
    for line in file_content.split('\n') {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        remove_end_of_line(&mut line.as_bytes().to_vec());
        patterns.push(IgnorePattern::new(line.to_string())?);
    }
    Ok(patterns)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_workspace_file() {
        let result = read_workspace_file("test.txt");
        assert!(result.is_ok());
        let content = result.unwrap();
        assert_eq!(content, b"Hello World!\r\n");
    }

    /// Content of .rgitignore:
    /// # Ignore target directory
    /// /target
    /// # Ignore .rgit directory
    /// /.rgit
    /// # Ignore .gitignore file
    /// .gitignore
    #[test]
    fn test_load_ignore_patterns() {
        let result = load_ignore_patterns();
        assert!(result.is_ok());
        let patterns = result.unwrap();
        assert_eq!(patterns.len(), 3);
        assert_eq!(patterns[0].get_pattern(), "./target");
        assert_eq!(patterns[0].is_dir(), true);
        assert_eq!(patterns[1].get_pattern(), "./.rgit");
        assert_eq!(patterns[1].is_dir(), true);
        assert_eq!(patterns[1].is_ext(), false);
        assert_eq!(patterns[2].get_pattern(), ".gitignore");
        assert_eq!(patterns[2].is_file(), true);
    }

    #[test]
    fn test_read_workspace_dir() {
        let result = read_workspace_dir("./test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"./test\\subtest\n./test\\test.txt\n".to_vec());
    }
}