use std::io::{self, Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha2::{Digest, Sha256};

use crate::object::header::Header;
use crate::object::object::Object;

pub struct Blob {
    header: Header,
    oid: String,
    content: Vec<u8>,
    is_compressed: bool,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Blob {
        let mut blob_content = content.clone();
        blob_content.push(b'\n');
        let header = Header::new(String::from("blob"), blob_content.len() as u64);
        Blob {
            header,
            oid: "\0".to_string(),
            content: blob_content,
            is_compressed: false,
        }
    }
}

impl Object for Blob {
    fn get_content(&self) -> &Vec<u8> {
        &self.content
    }

    fn get_oid(&self) -> &String {
        &self.oid
    }

    fn get_header(&self) -> &Header {
        &self.header
    }

    fn get_is_compressed(&self) -> &bool {
        &self.is_compressed
    }

    fn generate_hashed_oid(&mut self) -> io::Result<()> {
        if self.is_compressed {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cannot generate hash for already compressed content",
            ));
        }
        let mut hasher = Sha256::new();
        hasher.update(self.serialize()?);
        let result = hasher.finalize();
        self.oid = format!("{:x}", result);
        Ok(())
    }

    fn add_header_to_content(&mut self) -> io::Result<()> {
        if self.is_compressed {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cannot generate hash for already compressed content",
            ));
        }
        let mut full_content = Vec::new();
        full_content.extend(self.header.serialize()?);
        full_content.extend(self.content.clone());
        self.content = full_content;
        Ok(())
    }

    fn compress_content(&mut self) -> io::Result<()> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&self.content).unwrap();
        let compressed_content = encoder.finish().unwrap();
        self.content = compressed_content;
        self.is_compressed = true;
        Ok(())
    }

    fn decompress_content(&mut self) -> io::Result<()> {
        let mut decoder = ZlibDecoder::new(self.content.as_slice());
        let mut decompressed_content = String::new();
        decoder.read_to_string(&mut decompressed_content)?;
        self.content = decompressed_content.as_bytes().to_vec();
        self.is_compressed = false;
        Ok(())
    }

    fn serialize(&self) -> io::Result<Vec<u8>> {
        let serialized = self.content.clone();
        Ok(serialized)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let content = b"hello".to_vec();
        let blob = Blob::new(content);
        assert_eq!(blob.get_header().get_object_type(), "blob");
        assert_eq!(blob.get_header().get_size(), &5);
        assert_eq!(blob.get_content(), &b"hello".to_vec());
        assert_eq!(blob.get_is_compressed(), &false);
    }

    #[test]
    fn test_generate_hashed_oid() {
        let mut blob = Blob::new(b"hello".to_vec());
        blob.add_header_to_content().unwrap();
        blob.generate_hashed_oid().unwrap();
        assert_eq!(
            blob.content,
            b"blob 6\0hello\n".to_vec()
        );
        let mut hasher = Sha256::new();
        hasher.update(b"blob 6\0hello\n");
        assert_eq!(
            blob.oid,
            format!("{:x}", hasher.finalize())
        );
    }

    #[test]
    fn test_add_header_to_content() {
        let mut blob = Blob::new(b"hello".to_vec());
        blob.add_header_to_content().unwrap();
        assert_eq!(
            blob.content,
            b"blob 6\0hello\n".to_vec()
        );
    }

    #[test]
    fn test_compress_content() {
        let mut blob = Blob::new(b"hello".to_vec());
        blob.compress_content().unwrap();
        assert_eq!(blob.get_is_compressed(), &true);
    }

    #[test]
    fn test_decompress_content() {
        let mut blob = Blob::new(b"hello".to_vec());
        blob.compress_content().unwrap();
        blob.decompress_content().unwrap();
        assert_eq!(blob.get_is_compressed(), &false);
    }

    #[test]
    fn test_serialize() {
        let blob = Blob::new(b"hello".to_vec());
        let serialized = blob.serialize().unwrap();
        assert_eq!(serialized, b"hello\n".to_vec());
    }
}
