#![allow(warnings)]
//#![warn(unused_imports)]

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::result::Result;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.bytes;
    }

    pub fn is_critical(&self) -> bool {
        //bit shift 1 by 5 to
        return self.bytes[0] & (1 << 5) == 0;
    }

    pub fn is_public(&self) -> bool {
        return self.bytes[1] & (1 << 5) == 0;
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        return self.bytes[2] & (1 << 5) == 0;
    }

    pub fn is_safe_to_copy(&self) -> bool {
        return self.bytes[3] & (1 << 5) == 1 << 5;
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

#[derive(Debug)]
pub enum ChunkTypeDecodingError {
    /// We found a bad byte while decoding. The u8 is the first invalid byte found.
    BadByte(u8),
    /// The chunk type to be decoded was the wrong size. The usize is the received size.
    BadLength(usize),
}

impl fmt::Display for ChunkTypeDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadByte(byte) => write!(f, "Bad byte: {byte} ({byte:b})", byte = byte),
            Self::BadLength(len) => write!(f, "Bad length: {} (expected 4)", len),
        }
    }
}

impl std::error::Error for ChunkTypeDecodingError {}

/* */
impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.bytes.iter() {
            write!(f, "{}", *byte as char)?;
        }
        Ok(())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Box<dyn Error>;
    fn try_from(chunk_type: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in chunk_type.iter() {
            if !(byte.is_ascii_uppercase() || byte.is_ascii_lowercase()) {
                return Err(Box::new(ChunkTypeDecodingError::BadByte(*byte)));
            }
        }
        Ok(Self { bytes: chunk_type })
    }
}

impl FromStr for ChunkType {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 4 {
            return Err(Box::new(ChunkTypeDecodingError::BadLength(s.len())));
        }
        let mut arr: [u8; 4] = [0, 0, 0, 0];
        let mut i = 0;
        for byte in bytes.iter() {
            if byte.is_ascii_uppercase() || byte.is_ascii_lowercase() {
                arr[i] = *byte;
            } else {
                return Err(Box::new(ChunkTypeDecodingError::BadByte(*byte)));
            }
            i += 1;
        }
        return Ok(Self { bytes: arr });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        for byte in actual.bytes() {
            println!("{}", byte);
        }
        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
