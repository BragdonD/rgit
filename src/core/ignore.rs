use std::io;
use std::fs;

pub struct IgnorePattern {
    is_dir: bool,
    is_file: bool,
    is_ext: bool,
    pattern: String,
}

impl IgnorePattern {
    pub fn new(pattern: String) -> io::Result<IgnorePattern> {
        let is_ext = pattern.starts_with('.'); 
        let mut pattern = pattern.trim().to_string();
        if pattern.starts_with('/') {
            pattern = ".".to_string() + &pattern;
        }
        let is_dir = fs::metadata(&pattern).unwrap().is_dir();
        let is_file = fs::metadata(&pattern).unwrap().is_file();
        Ok(IgnorePattern {
            is_dir,
            is_file,
            is_ext,
            pattern,
        })
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn is_file(&self) -> bool {
        self.is_file
    }

    pub fn is_ext(&self) -> bool {
        self.is_ext
    }

    pub fn get_pattern(&self) -> &String {
        &self.pattern
    }

    pub fn is_match(&self, file_path: &str) -> bool {
        if self.is_dir {
            return file_path.starts_with(&self.pattern);
        }
        if self.is_file { 
            // File match needs to be absolute
            // Else it will match any file with the same name (unwanted)
            return file_path.eq(&self.pattern);
        }
        if self.is_ext {
            return file_path.ends_with(&self.pattern);
        }
        false
    }
}

pub fn should_ignore(path: &str, ignore_patterns: &Vec<IgnorePattern>) -> bool {
    for pattern in ignore_patterns {
        if pattern.is_match(path) {
            return true;
        }
    }
    false
}

pub fn default_ignore(path: &str) -> bool {
    if path.starts_with(".git") || path.starts_with(".rgit") {
        return true;
    }
    false
}