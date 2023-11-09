use actix_web::{web, Result};
use reqwest::Client;
//use std::error::Error;
use crate::models;
use quick_xml::se::to_string;
use std::fmt;
use crate::compression;

// Create a custom error type
#[derive(Debug)]
struct CustomError(String);

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

pub async fn credit_req(
    data: models::ReqPay,
    client: &Client,
    app_data: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    // let go_server =  std::env::var("GO_SERVER");
    // let port = std::env::var("PORT");
    //println!("Credit req :6");
    // let validated_xml = match to_string(payer) {
    //     Ok(xml) => xml,
    //     Err(e) => {
    //         // Handle the serialization error and convert it to a CustomError
    //         let error_msg = format!("Serialization error: {:?}", e);
    //         return Err(Box::new(CustomError(error_msg)));
    //     }
    // };

    // let client = Client::new();

    // let url = std::env::var("CREDIT_REQ").unwrap_or_default();
    // let url = app_data;
    //println!("credit{}", url);
    //let url = "http://192.168.68.123:8080/creditreq";
    //println!("url = {:?}", url);
    // Send the XML data in the request body

    let xml_string = to_string(&data).unwrap();
    let bytes = xml_string.into_bytes();
    let compressed_data = compression::compress_data( &bytes );

    let response = match client
        .post(app_data)
        .header("Content-Type", "application/xml")
        .body(compressed_data)
        .send()
        .await
    {
        Ok(response) => {
            print!("{}", response.url());
            response
        }
        Err(e) => {
            // Handle the reqwest::Error and convert it to a CustomError
            let error_msg = format!("Request error: {:?}", e);
            return Err(Box::new(CustomError(error_msg)));
        }
    };

    // Check for request success and handle the response
    if response.status().is_success() {
        // Successfully validated, use validated_xml
        let response_body = response.text().await?;
        //println!("Response from credit req {}: {}", url, response_body);
        Ok("Credit Req Done".to_string())
    } else {
        // Handle the error, e.g., return an error response
        Err(Box::new(CustomError("Validation failed".to_string())))
    }
}
