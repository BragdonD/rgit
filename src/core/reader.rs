use std::fs::{File, self};
use std::io::{self, Read};
use crate::core::ignore::IgnorePattern;

pub fn read_workspace_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
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
        println!("Ignore pattern: {}", line);
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
}