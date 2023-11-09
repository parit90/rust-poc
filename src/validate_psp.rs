// use actix_web::{web, Result, web::Data};
// use reqwest::Client;
// use std::sync::Mutex;
// //use std::error::Error;
// use std::fmt;
// use lazy_static::lazy_static;
// use crate::MyURLs;
// //use crate::Mutex;

// use crate::CLIENT;
// use quick_xml::se::to_string;
use crate::models;
use crate::compression;

// // Create a custom error type
// #[derive(Debug)]
// struct CustomError(String);

// impl std::error::Error for CustomError {}

// impl fmt::Display for CustomError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "CustomError: {}", self.0)
//     }
// }

// pub async fn validate_psp(data: models::ReqPay, client: &Client, app_data: &String) -> Result<String, Box<dyn std::error::Error>> {
//     println!("validate_psp:: 2");
//     let validated_xml = match to_string(&data) {
//         Ok(xml) => xml,
//         Err(e) => {
//             // Handle the serialization error and convert it to a CustomError
//             let error_msg = format!("Serialization error: {:?}", e);
//             println!("errror....");
//             return Err(Box::new(CustomError(error_msg)));
//         }
//     };

//     let url = app_data; //std::env::var("VALIDATE_PSP").unwrap_or_default();
//     //println!("client {}",client);
//     // Send the XML data in the request body
//     let response = match client
//         .post(url)
//         .header("Content-Type", "application/xml")
//         .body(data.clone())
//         .send()
//         .await
//     {
//         Ok(response) => response,
//         Err(e) => {
//             // Handle the reqwest::Error and convert it to a CustomError
//             let error_msg = format!("Request error: {:?}", e);
//             return Err(Box::new(CustomError(error_msg)));
//         }
//     };

//     // Check for request success and handle the response
//     if response.status().is_success() {
//         // Successfully validated, use validated_xml
//         let response_body = response.text().await?;
//         println!("validated_psp Response {}", url);
//         Ok("Validated PSP Done".to_string())

//     } else {
//         // Handle the error, e.g., return an error response
//         Err(Box::new(CustomError("Validation failed".to_string())))
//     }
// }

use quick_xml::se::to_string;
use reqwest::Client;
use std::fmt;

// Create a custom error type
#[derive(Debug)]
pub struct CustomError(String);

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

pub async fn validate_psp(
    data: models::ReqPay,
    client: &Client,
    app_data: &String,
) -> Result<String, CustomError> {
    let validated_xml =
        to_string(&data).map_err(|e| CustomError(format!("Serialization error: {:?}", e)))?;

    //let url = app_data;

    // compress the current data
    // let data_string = to_string(&data).unwrap();
    let xml_string = to_string(&data).unwrap();
    let bytes = xml_string.into_bytes();
    let compressed_data = compression::compress_data( &bytes );


    let response = client
        .post(app_data)
        .header("Content-Type", "application/xml")
        .body(compressed_data)
        .send()
        .await
        .map_err(|e| CustomError(format!("Request error: {:?}", e)))?;
    

    if response.status().is_success() {
        let response_body = response.text().await;
        Ok("Validated PSP Done".to_string())
    } else {
        Err(CustomError("Validation failed".to_string()))
    }
}
