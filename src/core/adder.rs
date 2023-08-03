use std::io;

use crate::object::{blob::Blob, object::Object, writer::write_object_to_file};
use super::reader::read_workspace_file;

pub fn add_file(file_path: &str, verbose: bool) -> io::Result<()> {
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