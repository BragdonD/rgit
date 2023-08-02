use std::fs::{File, create_dir_all};
use std::io::{self, Write};
use std::path;
use crate::object::object::Object;

pub fn write_object_to_file(obj: &dyn Object) -> io::Result<()> {
    let mut file_path = String::from(".rgit/objects/");
    file_path.push_str(&obj.get_oid()[0..2]);
    file_path.push_str("/");
    file_path.push_str(&obj.get_oid()[2..]);
    create_dir_all(path::Path::new(&file_path).parent().unwrap())?;
    let mut file = File::create(file_path)?;
    file.write_all(&obj.serialize()?)?;
    Ok(())
}