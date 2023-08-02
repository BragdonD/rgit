use std::fs::File;
use std::io::{self, Read};
use flate2::read::ZlibDecoder;

pub fn read_object_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    buffer = buffer[0..buffer.len()-1].to_vec();
    Ok(buffer)
}

pub fn uncompressed_object_content(content: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(&content[..]);
    let mut uncompressed_content = Vec::new();
    decoder.read_to_end(&mut uncompressed_content)?;
    Ok(uncompressed_content)
}