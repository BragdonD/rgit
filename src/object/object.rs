use std::io;
use crate::object::header::Header;

pub trait Object {
    fn get_header(&self) -> &Header;
    fn get_oid(&self) -> &String;
    fn get_content(&self) -> &Vec<u8>;
    fn get_is_compressed(&self) -> &bool;
    fn generate_hashed_oid(&mut self) -> io::Result<()>;
    fn add_header_to_content(&mut self) -> io::Result<()>;
    fn compress_content(&mut self) -> io::Result<()>;
    fn decompress_content(&mut self) -> io::Result<()>;
    fn serialize(&self) -> io::Result<Vec<u8>>;
}