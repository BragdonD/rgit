use std::io;

pub struct Header {
    object_type: String,
    size: u64,
}

impl Header {
    pub fn new(object_type: String, size: u64) -> Header {
        Header {
            object_type,
            size,
        }
    }

    pub fn get_object_type(&self) -> &String {
        &self.object_type
    }

    pub fn get_size(&self) -> &u64 {
        &self.size
    }

    pub fn serialize(&self) -> io::Result<Vec<u8>> {
        let mut serialized = Vec::new();
        serialized.extend(self.object_type.as_bytes());
        serialized.extend(" ".as_bytes());
        serialized.extend(self.size.to_string().as_bytes());
        serialized.extend("\0".as_bytes());
        Ok(serialized)
    }
}