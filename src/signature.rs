use actix_web::{web, Result, web::{Data, Buf}};
use futures::lock::Mutex;
use reqwest::Client;
//use std::error::Error;
use crate::models;
use quick_xml::se::to_string;
use std::fmt;
use std::sync::Arc;


// Create a custom error type
#[derive(Debug)]
struct CustomError(String);

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

pub async fn get_signature(
    data: models::ReqPay,//Arc<Mutex<web::Bytes>>,
    client: &Client,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("I am in get signature fn");
    let url = "http://0.0.0.0:8082/generatesignature";
    // let data = data.lock().await;
    // Send the XML data in the request body
    let response = match client
        .post(url)
        .header("Content-Type", "application/xml")
        .body(data)
        .send()
        .await
    {
        Ok(response) => {
            ////println!("response debit req {}", response.url());
            println!("I am Here");
            response
        }
        Err(e) => {
            println!("The error is {:?}",e);
            // Handle the reqwest::Error and convert it to a CustomError
            let error_msg = format!("Request error: {:?}", e);
            return Err(Box::new(CustomError(error_msg)));
        }
    };
    // Check for request success and handle the response
    if response.status().is_success() {
        // Successfully validated, use validated_xml
        let response_body = response.text().await?;
        println!("{:?}",response_body);
        //println!("Response from debit req {}", url);
        Ok(response_body)
    } else {
        println!("The error is 2");
        // Handle the error, e.g., return an error response
        Err(Box::new(CustomError("Validation failed".to_string())))
    }
}
