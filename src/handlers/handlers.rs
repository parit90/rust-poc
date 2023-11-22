use actix_web::{web, HttpResponse, Result};
use futures::join;
// use serde_xml_rs::de::from_str;
// use serde_xml_rs::to_string;
use std::sync::Arc;
use tokio::task::spawn;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

use log::info;

use crate::utils::{make_request , get_signature};
use crate::validation::validate_parameters;

use crate::models::ReqPay;


pub async fn handle_reqpay(data: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
    let xml_data = String::from_utf8(data.to_vec())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid UTF-8 data"))?;

    let req_pay: Result<ReqPay, _> = from_str(&xml_data);

    match req_pay {
        Ok(req_pay) => {
            
            let reqpay = req_pay.clone();
            let req_pay = Arc::new(req_pay);

            let (validation_result, signature) = join!(
                validate_parameters(&req_pay),
                get_signature(data.clone(), true)
            );

            info!("signature {:?}", signature);
            
            if let Err(_) = validation_result {
                return Ok(HttpResponse::BadRequest().body("Invalid parameters"));
            }

              // Validation succeeded, spawn make_request in the background
              spawn(async move {
                // let body_string = to_string(&reqpay).unwrap();
                
                // let body_string = String::from("test");
                // println!("{:?}", body_string);

                if let Err(api_err) = make_request("http://10.166.0.8:8081/reqauthdetails", "POST", xml_data).await {
                    eprintln!("Error calling external API in the background: {:?}", api_err);
                    // Handle the error as needed
                }
              });

            Ok(HttpResponse::Ok().body("OK"))
        }
        Err(err) => {
            eprintln!("Error while deserializing XML: {:?}", err);
            Ok(HttpResponse::InternalServerError().body("Error while deserializing XML"))
        }
    }
}

pub async fn handle_resp_auth_details(data: web::Bytes) -> Result<HttpResponse, actix_web::Error> {

    info!("called {:?}", "/reqauthdetails");
    let xml_data = String::from_utf8(data.to_vec())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid UTF-8 data"))?;

    let req_pay: Result<ReqPay, _> = from_str(&xml_data);

    match req_pay {
        Ok(req_pay) => {
            let reqpay = req_pay.clone();
            let req_pay = Arc::new(req_pay);
            
            let (validation_result, signature) = join!(
                validate_parameters(&req_pay),
                get_signature(data.clone(), true)
            );

            info!("signature {:?}", signature);
            
            if let Err(_) = validation_result {
                return Ok(HttpResponse::BadRequest().body("Invalid parameters"));
            }else{

              // Validation succeeded, spawn make_request in the background
                spawn(async move {
                    // let body_string = to_string(&reqpay).unwrap();
                    if let Err(api_err) = make_request("http://10.166.0.8:8081/debitreq", "POST", xml_data).await {
                        eprintln!("Error calling external API in the background: {:?}", api_err);
                        // Handle the error as needed
                    }
                  });
            }

             
            Ok(HttpResponse::Ok().body("OK"))
        }
        Err(err) => {
            eprintln!("Error while deserializing XML: {:?}", err);
            Ok(HttpResponse::InternalServerError().body("Error while deserializing XML"))
        }
    }
}

pub async fn handle_debit_resp(data: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
    let xml_data = String::from_utf8(data.to_vec())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid UTF-8 data"))?;

    let req_pay: Result<ReqPay, _> = from_str(&xml_data);

    match req_pay {
        Ok(req_pay) => {
            let reqpay = req_pay.clone();
            let req_pay = Arc::new(req_pay);
            let (validation_result, signature) = join!(
                validate_parameters(&req_pay),
                get_signature(data.clone(), true)
            );

            info!("signature {:?}", signature);
            
            if let Err(_) = validation_result {
                return Ok(HttpResponse::BadRequest().body("Invalid parameters"));
            }

              // Validation succeeded, spawn make_request in the background
              spawn(async move {
                // let body_string = to_string(&reqpay).unwrap();
                if let Err(api_err) = make_request("http://10.166.0.8:8082/creditreq", "POST", xml_data).await {
                    eprintln!("Error calling external API in the background: {:?}", api_err);
                    // Handle the error as needed
                }
              });

            Ok(HttpResponse::Ok().body("OK"))
        }
        Err(err) => {
            eprintln!("Error while deserializing XML: {:?}", err);
            Ok(HttpResponse::InternalServerError().body("Error while deserializing XML"))
        }
    }
}

pub async fn handle_credit_resp(data: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
    let xml_data = String::from_utf8(data.to_vec())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid UTF-8 data"))?;

    let req_pay: Result<ReqPay, _> = from_str(&xml_data);

    match req_pay {
        Ok(req_pay) => {
            let reqpay = req_pay.clone();
            let req_pay = Arc::new(req_pay);
            let (validation_result, signature) = join!(
                validate_parameters(&req_pay),
                get_signature(data.clone(), true)
            );

            println!("signature {:?}", signature);
            
            if let Err(_) = validation_result {
                return Ok(HttpResponse::BadRequest().body("Invalid parameters"));
            }

              // Validation succeeded, spawn make_request in the background
              spawn(async move {
                // let body_string = to_string(&reqpay).unwrap();
                if let Err(api_err) = make_request("http://10.166.0.8:8082/reqtxconfirm", "POST", xml_data).await {
                    eprintln!("Error calling external API in the background: {:?}", api_err);
                    // Handle the error as needed
                }
              });

            Ok(HttpResponse::Ok().body("OK"))
        }
        Err(err) => {
            eprintln!("Error while deserializing XML: {:?}", err);
            Ok(HttpResponse::InternalServerError().body("Error while deserializing XML"))
        }
    }
}

pub async fn handle_resp_txn_confirm(data: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
    let xml_data = String::from_utf8(data.to_vec())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid UTF-8 data"))?;

    let req_pay: Result<ReqPay, _> = from_str(&xml_data);

    match req_pay {
        Ok(req_pay) => {

            let reqpay = req_pay.clone();
            let req_pay = Arc::new(req_pay);
            let validation_result = validate_parameters(&req_pay).await;

            if let Err(_) = validation_result {
                return Ok(HttpResponse::BadRequest().body("Invalid parameters"));
            }

            spawn(async move {
                // let body_string = to_string(&reqpay).unwrap();
                if let Err(api_err) = make_request("http://10.166.0.8:8081/resppay", "POST", xml_data).await {
                    eprintln!("Error calling external API in the background: {:?}", api_err);
                    // Handle the error as needed
                }
              });

            info!("{}", "successfully Txn confirmed");

        
            Ok(HttpResponse::Ok().body("OK"))
        }
        Err(err) => {
            eprintln!("Error while deserializing XML: {:?}", err);
            Ok(HttpResponse::InternalServerError().body("Error while deserializing XML"))
        }
    }
}
