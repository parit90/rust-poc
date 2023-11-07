use actix_web::{web, Result};
use reqwest::Client;
//use std::error::Error;
use crate::models;
use quick_xml::se::to_string;
use std::fmt;

// Create a custom error type
#[derive(Debug)]
struct CustomError(String);

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

pub async fn resp_pay(
    data: models::ReqPay,
    client: &Client,
    app_data: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let validated_xml = match to_string(&data) {
        Ok(xml) => xml,
        Err(e) => {
            // Handle the serialization error and convert it to a CustomError
            let error_msg = format!("Serialization error: {:?}", e);
            return Err(Box::new(CustomError(error_msg)));
        }
    };

    // let client = Client::new();
    // let url = std::env::var("RESP_PAY").unwrap_or_default(); //"http://192.168.68.123:8080/resppay";
    // let url = app_data;
    //println!("resp_pay ---->{}", url);

    // Send the XML data in the request body
    let response = match client
        .post(app_data)
        .header("Content-Type", "application/xml")
        .body(data)
        .send()
        .await
    {
        Ok(response) => response,
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
        //println!("Response from validated_psp {}: {}", url, response_body);
        Ok("response Pay Done".to_string())
    } else {
        // Handle the error, e.g., return an error response
        Err(Box::new(CustomError("Validation failed".to_string())))
    }
}

pub async fn req_tx_confirm(
    data: models::ReqPay,
    client: &Client,
    app_data: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let validated_xml = match to_string(&data) {
        Ok(xml) => xml,
        Err(e) => {
            // Handle the serialization error and convert it to a CustomError
            let error_msg = format!("Serialization error: {:?}", e);
            return Err(Box::new(CustomError(error_msg)));
        }
    };

    // let client = Client::new();
    // let url = std::env::var("REQ_TX_CONFIRM").unwrap_or_default();//"http://192.168.68.123:8080/reqtxconfirm";
    let url = app_data;
    //println!("req_tx_confirm ----> {}", url);

    // Send the XML data in the request body
    let response = match client
        .post(url)
        .header("Content-Type", "application/xml")
        .body(data)
        .send()
        .await
    {
        Ok(response) => response,
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
        //println!("Response from validated_psp {}: {}", url, response_body);
        Ok("req_tx_confirm".to_string())
    } else {
        // Handle the error, e.g., return an error response
        Err(Box::new(CustomError("Validation failed".to_string())))
    }
}
