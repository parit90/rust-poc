use actix_web::{web, Result};
use sha2::{Sha256, Digest};
use rsa::{Pkcs1v15Encrypt, RsaPublicKey, pkcs8:: DecodePublicKey};
use std::sync::Once;
use std::error::Error;



static mut PUBLIC_KEY: Option<RsaPublicKey> = None;
static INIT: Once = Once::new();

fn initialize_public_key() {
    INIT.call_once(|| {
        // Initialize the public key only once
        unsafe {
            match RsaPublicKey::read_public_key_pem_file("public_key.pem") {
                Ok(key) => PUBLIC_KEY = Some(key),
                Err(err) => eprintln!("Error reading public key: {:?}", err),
            }
        }
    });
}

fn compute_sha256(data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());

    Ok(hasher.finalize().to_vec())
}


fn encrypt(data: &[u8]) -> Option<Vec<u8>> {
    initialize_public_key(); // Ensure the public key is initialized

    let mut rng = rand::thread_rng();
    let pub_key = unsafe { PUBLIC_KEY.clone()? };
    Some(pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).ok()?)
}

// pub async fn get_signature(data: web::Bytes,enable_signature: bool) -> Vec<u8> {

//     if !enable_signature {
//       return Vec::new() ; 
//     }

//     let _data = String::from_utf8(data.to_vec()).unwrap();
//     let signature = match compute_sha256(&_data) {
//         Ok(sig) => sig,
//         Err(err) => {
//             eprintln!("Error computing SHA256: {:?}", err);
//             Vec::new() // Return an empty vector or handle the error accordingly
//         }
//     };

//     match encrypt(&signature) {
//         Some(result) => result,
//         None => {
//             eprintln!("Error encrypting signature");
//             Vec::new() // Return an empty vector or handle the error accordingly
//         }
//     }
// }


pub async fn get_signature(data: web::Bytes,enable_signature: bool) -> String {
    if !enable_signature {
      return String::new() ; 
    }
    let _data = String::from_utf8(data.to_vec()).unwrap();
    let signature = match compute_sha256(&_data) {
        Ok(sig) => sig,
        Err(err) => {
            eprintln!("Error computing SHA256: {:?}", err);
            return String::new() // Return an empty vector or handle the error accordingly
        }
    };

    match encrypt(&signature) {
        Some(encrypted_signature) =>{
            let signature_string: String = encrypted_signature.iter().map(|byte| format!("{:02X}", byte)).collect();
            signature_string
        } ,
        None => {
            eprintln!("Error encrypting signature");
            String::new() // Return an empty vector or handle the error accordingly
        }
    }


}



// let enable_signature = true;  // Set this based on your switch logic
// let signature = get_signature(data, enable_signature).await;



