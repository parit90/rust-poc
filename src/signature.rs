use actix_web::{web, Result, web::{Data, Buf}};
use futures::lock::Mutex;
use reqwest::Client;
//use std::error::Error;
use crate::models;
use quick_xml::se::to_string;
use std::fmt;
use std::sync::Arc;
use sha2::{Sha256, Digest};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs8::{EncodePrivateKey, LineEnding, EncodePublicKey, DecodePublicKey, DecodePrivateKey}};


// Create a custom error type
#[derive(Debug)]
struct CustomError(String);

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

fn compute_sha256(data: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    // Finalize the hash and get the result as a byte array
    Ok(format!("{:x}", hasher.finalize()))
}

fn encrypt(data:&[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let pub_key: RsaPublicKey =  RsaPublicKey::read_public_key_pem_file("pub_key.pem").unwrap();
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    enc_data
}

fn Decrypt(data:&[u8],enc_data:Vec<u8>) {
    let mut rng = rand::thread_rng();
    let priv_key: RsaPrivateKey =  RsaPrivateKey::read_pkcs8_pem_file("priv_key.pem").unwrap();
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    println!("{:?}",dec_data);
    println!("{:?}",data);
    assert_eq!(&data[..], &dec_data[..]);   
}

pub async fn get_signature(
    data: web::Bytes,//Arc<Mutex<web::Bytes>>,
    client: &Client,
    app_data: &String,
) -> Vec<u8> {

    let _data = String::from_utf8(data.to_vec()).unwrap();

    let signature = compute_sha256(&_data).unwrap();

    // let chars: Vec<char> = encrypt(signature.as_bytes()).into_iter().map(|byte| byte as char).collect();

    // let result: String = chars.into_iter().collect();

    let result = encrypt(signature.as_bytes());

    // let data = data.lock().await;
    // Send the XML data in the request body
    // let response = match client
    //     .post(app_data)
    //     .header("Content-Type", "application/xml")
    //     .body(data)
    //     .send()
    //     .await
    // {
    //     Ok(response) => {
    //         ////println!("response debit req {}", response.url());
    //         response
    //     }
    //     Err(e) => {
    //         // Handle the reqwest::Error and convert it to a CustomError
    //         let error_msg = format!("Request error: {:?}", e);
    //         return Err(Box::new(CustomError(error_msg)));
    //     }
    // };
    // Check for request success and handle the response
    // if response.status().is_success() {
    //     // Successfully validated, use validated_xml
    //     let response_body = response.text().await?;
    //     //println!("Response from debit req {}", url);
    //     Ok(response_body)
    // } else {
    //     // Handle the error, e.g., return an error response
    //     Err(Box::new(CustomError("Validation failed".to_string())))
    // }
    result
}
