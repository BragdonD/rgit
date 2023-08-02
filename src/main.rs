use std::io;
use object::object::Object;

pub(crate) mod object;
pub(crate) mod core;

fn main() -> io::Result<()> {
    let file_content = core::reader::read_workspace_file("test.txt");
    let mut blob = object::blob::Blob::new(file_content.unwrap());
    blob.add_header_to_content().unwrap();
    blob.generate_hashed_oid().unwrap();
    blob.compress_content().unwrap();
    object::writer::write_object_to_file(&blob).unwrap();
    Ok(())
}
