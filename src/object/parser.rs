use std::io;
use std::str;
use super::{header::Header, object};

/// An object file will always contain the following:
/// 1. A header
/// 2. A null byte
/// 3. The content of the object

/// The content of the object will always be in the following format:
/// 1. The object type (removed in the parse_object_file function)
/// 2. A space
/// 3. The content length
/// 4. A null byte
pub fn extract_real_content_length(content: Vec<u8>) -> io::Result<u64>  {
    let mut content_length_str = Vec::new();
    for byte in content {
        if b" ".contains(&byte) {
            continue; // If byte is a space, then we have reached the end of the object type
        }
        if !b"0123456789".contains(&byte) {
            break; // If byte is not a number, then we have reached the end of the content length
        }
        content_length_str.push(byte);
        println!("{:?}", content_length_str);
    }
    let content_length_str = String::from_utf8(content_length_str)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let content_length = content_length_str.parse::<u64>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(content_length)
}

pub fn parse_object_file(content: Vec<u8>) -> io::Result<(Header, String)> {
    let object_type = str::from_utf8(&content[0..4])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    if !(object_type.contains("blob") || object_type.contains("tree") || object_type.contains("commit")) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid object type"));
    }
    let content_length = extract_real_content_length(content[object_type.len()..].to_vec())?;
    let real_content = content[object_type.len() + content_length.to_string().len() + 2..].to_vec(); // 2 is for the space and null byte
    let header = Header::new(String::from(object_type), content_length);
    let real_content = String::from_utf8(real_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok((header, real_content))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_real_content_length() {
        let content = b"12345\0".to_vec();
        let result = extract_real_content_length(content).unwrap();
        assert_eq!(result, 12345);
    }

    #[test]
    fn test_parse_object_file() {
        let content = b"blob 5\0hello".to_vec();
        let (header, real_content) = parse_object_file(content).unwrap();
        assert_eq!(header.get_object_type(), "blob");
        assert_eq!(header.get_size(), &5);
        assert_eq!(real_content, "hello");
    }
}

