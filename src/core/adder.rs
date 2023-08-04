use std::fs::{self};
use std::io;

use walkdir::WalkDir;

use super::ignore::{default_ignore, should_ignore, IgnorePattern};
use super::reader::{load_ignore_patterns, read_workspace_file};
use crate::object::{blob::Blob, object::Object, writer::write_object_to_file};

fn add_file(
    file_path: &str,
    verbose: bool,
    ignore_patterns: &Vec<IgnorePattern>,
) -> io::Result<()> {
    if should_ignore(file_path, ignore_patterns) || default_ignore(file_path) {
        return Ok(());
    }
    let file_content = read_workspace_file(file_path)?;
    let mut blob = Blob::new(file_content);
    blob.add_header_to_content().unwrap();
    blob.generate_hashed_oid().unwrap();
    blob.compress_content().unwrap();
    write_object_to_file(&blob).unwrap();
    if verbose {
        println!("Added file: {}", file_path);
    }
    Ok(())
}

fn add_directory_as_object(path: &str, verbose: bool, ignore_patterns: &Vec<IgnorePattern>) -> io::Result<()> {
    if should_ignore(path, ignore_patterns) || default_ignore(path) {
        return Ok(());
    }
    //let directory_content_as_str;

    if verbose {
        println!("Added directory: {}", path);
    }
    Ok(())
}

fn add_directory(
    path: &str,
    arguments: &[String],
    ignore_patterns: &Vec<IgnorePattern>,
) -> io::Result<()> {
    if should_ignore(path, ignore_patterns) || default_ignore(path) {
        return Ok(());
    }
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if should_ignore(entry.path().to_str().unwrap(), ignore_patterns) || default_ignore(path) {
            continue;
        }
        if entry.metadata().unwrap().is_dir() {
            // create a tree object (not implemented yet)
        } else if entry.metadata().unwrap().is_file() {
            add_file(
                entry.path().to_str().unwrap(),
                arguments.contains(&"--verbose".to_string()),
                ignore_patterns,
            )?;
        }
    }
    Ok(())
}

pub fn add(path: &str, arguments: &[String]) -> io::Result<()> {
    let ignore_patterns = load_ignore_patterns()?;

    if !arguments.contains(&"--force".to_string()) {
        for pattern in &ignore_patterns {
            if pattern.is_match(path) {
                return Ok(());
            }
        }
    }

    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        add_directory(path, arguments, &ignore_patterns)?;
    } else if metadata.is_file() {
        add_file(
            path,
            arguments.contains(&"--verbose".to_string()),
            &ignore_patterns,
        )?;
    }
    Ok(())
}
