use reqwest::{Client, Error as ReqwestError};
use serde::Serialize;
use serde_xml_rs::to_string;
use std::{sync::{Once, Arc}, fmt::Debug};
use log::info;
use std::fmt;

use crate::models::ReqPay;

lazy_static::lazy_static! {
    static ref CLIENT: Client = Client::new();
    static ref INIT: Once = Once::new();
}

fn init_client() {
    INIT.call_once(|| {
        let _ = CLIENT;
    });
}


#[derive(Debug)]
pub struct CustomError(String);

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}


pub async fn make_request(
    url: &str,
    method: &str,
    body:  String,
) -> Result<String, CustomError> {
    
    init_client();
    info!("calling {:?}", url);
    
    // let body_string = to_string(&body).map_err(|e| CustomError(format!("Serialization error: {:?}", e)))?;


    
    let response = CLIENT
        .request(reqwest::Method::from_bytes(method.as_bytes()).unwrap(), url)
        .header("Content-Type", "application/xml")
        .body(body)
        .send()
        .await
        .map_err(|e| CustomError(format!("Request error: {:?}", e)))?;

    let response = match response.error_for_status() {
        Ok(response) => response,
        Err(err) => return Err(CustomError("Validation failed".to_string()))
    };

    let body = response.text().await.map_err(|e| CustomError(format!("Serialization error: {:?}", e)))?;
    Ok(body)
}



// pub async fn make_http_request<T: Serialize>(
//     url: &str,
//     method: &str,
//     body:  T,
// ) -> Result<String, ReqwestError> {
//     init_client();

//     info!("calling {:?}", url);

//     let body_string =  to_string(&body).unwrap();
    
//     let response = CLIENT
//         .request(reqwest::Method::from_bytes(method.as_bytes()).unwrap(), url)
//         .header("Content-Type", "application/xml")
//         .body(body_string)
//         .send()
//         .await?;

//     let response = match response.error_for_status() {
//         Ok(response) => response,
//         Err(err) => return Err(err.into()),
//     };

//     let body = response.text().await?;
//     Ok(body)
// }


/*** 
   let url = "https://example.com";
    let xml_body = r#"<your_xml_payload_here>"#;
    let result = http_utils::make_request(url, "POST", xml_body).await;

    match result {
        Ok(response) => {
            println!("Response: {}", response);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
 */