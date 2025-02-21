#![allow(unused_imports)]

use crate::chunk_type::ChunkType;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io::{BufReader, Read};

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Self {
        Chunk {
            length: chunk_data.len() as u32,
            chunk_type: chunk_type.clone(),
            chunk_data: chunk_data.clone(),
            crc: crc::crc32::checksum_ieee(
                &[chunk_type.bytes().as_slice(), chunk_data.as_slice()].concat(),
            ),
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;
    fn try_from(chunk: &[u8]) -> Result<Self, Self::Error> {
        //length is first four bytes of array
        //need to check if overflow
        if chunk.len() < 8 {
            return Err(Box::new(ChunkError {
                err: "too short".to_owned(),
            }));
        }
        let buf: [u8; 4] = <[u8; 4]>::try_from(&chunk[0..4]).unwrap();
        let len: u32 = u32::from_be_bytes(buf);

        //next four bytes are chunk type
        let chunk_type: [u8; 4] = <[u8; 4]>::try_from(&chunk[4..8]).unwrap();
        let chunk_type_fin = ChunkType::try_from(chunk_type)?;

        //every byte after is chunk data
        let mut i = 0;
        let mut v: Vec<u8> = Vec::new();
        for byte in chunk.iter() {
            if i >= 8 {
                v.push(*byte);
            }
            i += 1;
        }

        //need to combine chunk type and chunk data into one array for the crc
        let crc_fin = crc::crc32::checksum_ieee(&[chunk_type.as_slice(), v.as_slice()].concat());

        Ok(Chunk {
            length: len,
            chunk_type: chunk_type_fin,
            chunk_data: v,
            crc: crc_fin,
        })
    }
}

#[derive(Debug)]
pub struct ChunkError {
    err: String,
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err)?;
        Ok(())
    }
}

impl std::error::Error for ChunkError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }
    /*
    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
        */
}
