use crate::debit_req;
use crate::models;
use crate::resp_pay;
use crate::{credit_req, MyURLs};
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use reqwest::Client;
use serde_xml_rs::from_str;
use tokio::spawn;
use std::sync::Arc;

pub async fn debit_resp_callback(
    data: web::Bytes,
    client: &Client,
    app_data: &String,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    // Deserialize the XML data into a Rust struct (e.g., models::respAuth::RespAuthDetail)
    let debit_req_data: models::ReqPay = from_str(std::str::from_utf8(&data)?)?;

    // Process the received RespAuthDetail
    // You can add your logic here to handle the received data

    // Example: Print the received data
    //println!("Received debit_resp_callback:");

    // let validate_task = spawn(async move {
    //     let _ = credit_req::credit_req(data, client).await;
    // });

    let client_clone = client.clone(); // Clone the client reference
    let app_data = Arc::new(app_data.clone());

    let validate_task = spawn(async move {
        if let Err(err) = credit_req::credit_req(debit_req_data, &client_clone, &app_data).await {
            eprintln!("Error in credit_req: {:?}", err);
        }
    });

    Ok(HttpResponse::Ok().finish())
    // You can send a response back to the provider if needed
    // match credit_req::credit_req(&debit_req_data).await {
    //     Ok(validated_xml) => {
    //         // Successfully validated, use validated_xml
    //         Ok(HttpResponse::Ok().finish())
    //     }
    //     Err(error) => {
    //         // Handle the error, e.g., return an error response
    //         Ok(HttpResponse::Ok().finish())
    //     }
    // }
}

pub async fn resp_auth_callback(
    data: web::Bytes,
    client: &Client,
    app_data: &String,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    //println!("resp_auth_callback: 3");
    // Deserialize the XML data into a Rust struct (e.g., models::respAuth::RespAuthDetail)
    // let data = String::from_utf8(data.to_vec()).expect("Failed to convert bytes to string");
    // let resp_auth_detail: models::ReqPay = from_str(&data).expect("Failed to parse XML data");
    
    let resp_auth_detail: models::ReqPay = from_str(std::str::from_utf8(&data)?)?;

    // Process the received RespAuthDetail
    // You can add your logic here to handle the received data

    // Example: Print the received data

    let client_clone = client.clone(); // Clone the client reference
    let app_data = Arc::new(app_data.clone());

    let validate_task = spawn(async move {
        let _ = debit_req::debit_req(resp_auth_detail, &client_clone, &app_data).await;
    });

    Ok(HttpResponse::Ok().finish())
    // You can send a response back to the provider if needed
    // match debit_req::debit_req(&resp_auth_detail).await {
    //     Ok(validated_xml) => {
    //         // Successfully validated, use validated_xml
    //         //println!("debit_req Response");
    //         Ok(HttpResponse::Ok().finish())
    //     }
    //     Err(error) => {
    //         //println!("debit_req Error {:?}", error);
    //         // Handle the error, e.g., return an error response
    //         Ok(HttpResponse::Ok().finish())
    //     }
    // }
}

pub async fn credit_resp_callback(
    data: web::Bytes,
    client: &Client,
    app_data: Data<MyURLs>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    //println!("credit_resp_callback");
    // Deserialize the XML data into a Rust struct (e.g., models::respAuth::RespAuthDetail)
    let data = String::from_utf8(data.to_vec()).expect("Failed to convert bytes to string");
    let debit_req_data: models::ReqPay = from_str(&data).expect("Failed to parse XML data");

    // Process the received RespAuthDetail
    // You can add your logic here to handle the received data

    let client_clone = client.clone(); // Clone the client reference
    let _client_clone = client.clone(); // Clone the client reference

    let app_data = app_data.clone();
    let _app_data = app_data.clone();

    let drd = debit_req_data.clone();

    let _ = spawn(async move {
        let _ = resp_pay::resp_pay(debit_req_data, &client_clone, &app_data.RESP_PAY_URL).await;
    });

    let _ = spawn(async move {
        let _ = resp_pay::req_tx_confirm(drd, &_client_clone, &_app_data.REQ_TX_CONFIRM_URL).await;
    });

    // Example: Print the received data
    // match resp_pay::resp_pay(&debit_req_data).await {
    //     Ok(validated_xml) => {
    //         // Successfully validated, use validated_xml
    //         //println!("resp_pay Response ");
    //         ()
    //     }
    //     Err(error) => {
    //         //println!("resp_pay Error {:?}", error);
    //         // Handle the error, e.g., return an error response
    //         ()
    //     }
    // }

    // match resp_pay::req_tx_confirm(&debit_req_data).await {
    //     Ok(validated_xml) => {
    //         // Successfully validated, use validated_xml
    //         //println!("ReqTxn Confirm");
    //         Ok(HttpResponse::Ok().finish())
    //     }
    //     Err(error) => {
    //         //println!("ReqTxn Confirm {:?}", error);
    //         // Handle the error, e.g., return an error response
    //         Ok(HttpResponse::Ok().finish())
    //     }
    // }
    Ok(HttpResponse::Ok().finish())
}

pub async fn res_tx_conf_callback(
    data: web::Bytes,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    // Example: Print the received data
    //println!("resp_auth_callback: ");
    Ok(HttpResponse::Ok().finish())
}
