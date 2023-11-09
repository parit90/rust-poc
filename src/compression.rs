use crate::models;
use actix_web::dev::Payload;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use serde_xml_rs::to_string;
use serde_xml_rs::from_str;
use serde_xml_rs::Error;
use serde::{Serialize, de::DeserializeOwned};

pub fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

pub fn decompress_data(data: &[u8]) -> Vec<u8> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    decompressed
}

pub fn compress_struct(payload : models::ReqPay) -> Vec<u8> {
    let xml_string = to_string(&payload).unwrap();
    let bytes = xml_string.into_bytes();
    let compressed_data = compress_data( &bytes );
    compressed_data
}



pub fn decompress_and_convert_to_struct(data: &[u8]) -> Result<models::ReqPay, Error> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    let xml_string = String::from_utf8_lossy(&decompressed).to_string();
    from_str(&xml_string)
}

