use crate::chunk::Chunk;
use crate::chunk::ChunkError;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::fs;
use std::str::FromStr;

pub fn encode(file_path: &str, chunk_type: &str, message: &str) {
    let mut image = Png::try_from(fs::read(file_path).unwrap().as_slice()).unwrap();
    let chunk_data = message.as_bytes().to_vec();
    let actual_chunk_type = ChunkType::from_str(chunk_type).expect("string not four characters");
    let secret_chunk = Chunk::new(actual_chunk_type, chunk_data);
    image.append_chunk(secret_chunk);
    fs::write(file_path, image.as_bytes()).unwrap();
}

pub fn decode(file_path: &str, chunk_type: &str) {
    let mut image = Png::try_from(fs::read(file_path).unwrap().as_slice()).unwrap();
    let encoded_chunk = image
        .chunk_by_type(chunk_type)
        .expect("incorrect chunk type");
    println!("{}", Chunk::data_as_string(encoded_chunk).unwrap());
}

pub fn remove(file_path: &str, chunk_type: &str) {
    let mut image = Png::try_from(fs::read(file_path).unwrap().as_slice()).unwrap();
    image.remove_first_chunk(chunk_type);
    fs::write(file_path, image.as_bytes()).unwrap();
}

pub fn print_chunks(file_path: &str) {
    let mut image = Png::try_from(fs::read(file_path).unwrap().as_slice()).unwrap();
    println!("{}", String::from_utf8(image.as_bytes()).unwrap());
}
