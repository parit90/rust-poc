// use actix_service::boxed::BoxService;
use actix_web::{web, App, HttpResponse, HttpServer, Result, web::{Data, Buf}, Error, dev::{Service,Transform,ServiceRequest, ServiceResponse}, HttpMessage, error::PayloadError, body::MessageBody};
use futures::{StreamExt, Stream};
use kafka::create_kafka_producer;
use openssl::{sha::Sha256, pkey::PKey};
use serde_xml_rs::{from_str, from_reader};
// use quick_xml::se::to_string;
use reqwest::{Client};
use std::{fmt, fs, pin::Pin};
//use std::io;
use dotenv::dotenv;
use tokio::spawn;
use lazy_static::lazy_static;
use std::sync::Mutex;

use std::fs::File;

mod models;
mod validate_psp;
mod debit_req;
mod credit_req;
mod resp_pay;
mod callback;
mod kafka;
mod signature;
extern crate num_cpus;

use rdkafka::error::KafkaError;
use rdkafka::producer::FutureProducer;
use std::sync::Arc;
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};
use openssl::hash::MessageDigest;

use actix_web_lab::middleware::{Next, from_fn};



// Create a custom error type
#[derive(Debug)]
struct CustomError(String);

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}


lazy_static! {
    static ref CLIENT: Client = Client::new();
    static ref KAFKA_PRODUCER: Mutex<Option<FutureProducer>> = Mutex::new(None);
}

pub async fn create_kafka_producer_if_needed() -> Result<FutureProducer, KafkaError> {
    // Check if the producer has already been created
    let mut producer = KAFKA_PRODUCER.lock().unwrap();
    
    if producer.is_none() {
        // Create a Kafka producer with the specified configuration
        let kafka_producer = create_kafka_producer().await?;
        *producer = Some(kafka_producer);
    }
    
    Ok(producer.as_ref().unwrap().clone())
}

async fn process_xml1(data: web::Bytes, app_data: Data<MyURLs>) -> HttpResponse {
    // ////println!("Process_xml:: 1 {:?}", app_data.VALIDATE_PSP_URL);
    // Deserialize the XML data into a Rust struct
    let data = String::from_utf8(data.to_vec()).expect("Failed to convert bytes to string");
    let req_pay: models::ReqPay = from_str(&data).expect("Failed to parse XML data");
    let payee = &req_pay;
    // let payee_xml = to_string(&payee).expect("Failed to serialize payee to XML");
    // let payer = &req_pay.payer;
    // // Process the user data as needed
    // //println!("Received XML data: {:?}", payee_xml);
    
    // let validate_task = spawn(async move {
    //     drop(validate_psp::validate_psp(req_pay, &CLIENT, &app_data.VALIDATE_PSP_URL).await);
    // });

    let validate_task = spawn(async move {
        if let Err(error) = validate_psp::validate_psp(req_pay, 
            &CLIENT,
            &app_data.VALIDATE_PSP_URL).await {
            eprintln!("Error while validating PSP: {:?}", error);
        }
    });
    
    HttpResponse::Ok().body("XML data processed successfully")
    // match validate_psp::validate_psp(payee).await {
    //     Ok(validated_xml) => {
    //         // Successfully validated, use validated_xml
    //         HttpResponse::Ok().body("XML data processed successfully")
    //     }
    //     Err(error) => {
    //         // Handle the error, e.g., return an error response
    //         HttpResponse::InternalServerError().body(format!("Error: {:?}", error))
    //     }
    // }
}
async fn process_xml(data: web::Bytes, app_data: Data<MyURLs>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    // Deserialize the XML data directly from a reader
    let reader = data.clone().reader();
    let req_pay: Result<models::ReqPay, serde_xml_rs::Error> = from_reader(reader);

    match req_pay {
        Ok(req_pay) => {
            // Clone the data for v
              // Create a Mutex to wrap the data
            let data_mutex = Arc::new(futures::lock::Mutex::new(data));
            
            // Clone the data_mutex for use in get_signature
        
            let signature = signature::get_signature(req_pay.clone(), &CLIENT).await;
            // Spawn the validation task
            let _validate_task = spawn(async move {
                if let Err(error) = validate_psp::validate_psp(
                    req_pay, 
                    &CLIENT,
                     &app_data.VALIDATE_PSP_URL).await {
                    eprintln!("Error while validating PSP: {:?}", error);
                }
            });

            // You can await the validation task here if you need to wait for it to complete
            // validate_task.await;

            Ok(HttpResponse::Ok().body("XML data processed successfully"))
        }
        Err(e) => {
            // Handle the XML deserialization error
            eprintln!("Error while deserializing XML: {:?}", e);
            // You can return an appropriate error response here
            Ok(HttpResponse::InternalServerError().body("Error while deserializing XML"))
        }
    }
}




async fn debit_resp_callback(data: web::Bytes, app_data: Data<MyURLs>) -> Result<HttpResponse, Box<dyn std::error::Error>>  {
    //println!("Debit resp callback : 5");
    callback::debit_resp_callback(data, &CLIENT, &app_data.CREDIT_REQ_URL).await
}

async fn resp_auth_callback(data: web::Bytes, app_data: Data<MyURLs>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    //println!("resp_auth_callback :3");
    callback::resp_auth_callback(data, &CLIENT, &app_data.DEBIT_REQ_URL).await
}


async fn credit_resp_callback(data: web::Bytes, app_data: Data<MyURLs>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    //println!("Credit resp callback 7");
    callback::credit_resp_callback(data, &CLIENT, app_data).await
}

async fn res_tx_conf_callback(
    data: web::Bytes,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    //println!("I am inside res_tx_conf_callback");
    // Call an asynchronous function and await its result
    let result = callback::res_tx_conf_callback(data.clone()).await;
   
    // Handle the result and create an appropriate response
    match result {
        Ok(_) => {
            // Call other asynchronous functions
            // let producer = match kafka::create_kafka_producer().await {
            //     Ok(producer) => producer,
            //     Err(err) => {
            //         eprintln!("Failed to create Kafka producer: {:?}", err);
            //         return Ok(HttpResponse::Ok().body("Error while producing to kafka"));
            //     }
            // };
            let producer = create_kafka_producer_if_needed().await?;
            let topic = "res-tx"; // Replace with your Kafka topic
            let payload = String::from_utf8(data.to_vec()).expect("Failed to convert bytes to string"); /* Create a payload */;

            // Send data to Kafka and handle any errors
            match kafka::send_to_kafka(&producer, topic, payload.as_bytes()).await {
                Ok(_) => {
                    // If everything is successful, return an HTTP response
                    Ok(HttpResponse::Ok().body("Success"))
                }
                Err(err) => {
                    // If there's an error with Kafka, return an error response
                    Err(err.into()) // Convert the Kafka error into the expected result type
                }
            }
        }
        Err(err) => {
            //println!("Error Happen: {:?}",err);
            // Handle the error from callback::res_tx_conf_callback
            Err(err.into()) // Convert the error into the expected result type
        }
    }
}

async fn testcallback() -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("Success"))
}
pub struct MyURLs {
    CREDIT_REQ_URL: String,
    DEBIT_REQ_URL: String,
    RESP_PAY_URL: String,
    REQ_TX_CONFIRM_URL: String,
    VALIDATE_PSP_URL: String,
}

// async fn generate_signature_middleware<S>(
//     req: ServiceRequest,
//     srv: BoxService<ServiceRequest, ServiceResponse, Error>,
// ) -> Result<ServiceResponse, Error>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
// {
//     // Generate the signature here
//     if let Some(payload) = req.extensions().get::<String>() {
//         if let Ok(signature) = generate_signature(payload) {
//             // Add the generated signature to the request's extensions
//             req.extensions_mut().insert(signature);
//         }
//     }

//     // Continue processing the request
//     let response = srv.call(req).await?;
//     Ok(response)
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let CREDIT_REQ_URL = std::env::var("CREDIT_REQ").unwrap_or_default(); 
    let DEBIT_REQ_URL = std::env::var("DEBIT_REQ").unwrap_or_default();
    let RESP_PAY_URL = std::env::var("RESP_PAY").unwrap_or_default();
    let REQ_TX_CONFIRM_URL = std::env::var("REQ_TX_CONFIRM").unwrap_or_default();
    let VALIDATE_PSP_URL = std::env::var("VALIDATE_PSP").unwrap_or_default();

    // let contents = fs::read_to_string("/Users/sahilpant/.ssh/id_rsa")
    //     .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    

    let MyURLs = Data::new(MyURLs { 
        CREDIT_REQ_URL,
        DEBIT_REQ_URL,
        RESP_PAY_URL,
        REQ_TX_CONFIRM_URL,
        VALIDATE_PSP_URL
    });

    std::env::set_var("RUST_LOG", "actix_web=debug");
    flame::start("main");

    HttpServer::new(move || {
        App::new()
        .app_data(Data::clone( &MyURLs))
        .service(
            web::resource("/respauth/callback")
            .route(web::post().to(resp_auth_callback))
        )
        .service(
            web::resource("/debitresp/callback")
            .route(web::post().to(debit_resp_callback))
        )
        .service(
            web::resource("/reqpay")
                .route(web::post().to(process_xml))
        )
        .service(
            web::resource("/creditresp/callback")
                .route(web::post().to(credit_resp_callback))
        )
        .service(
            web::resource("/restxconfirm/callback")
                .route(web::post().to(res_tx_conf_callback))
        )
        .service(
            web::resource("/test/get")
                .route(web::get().to(testcallback))
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    
    Ok(())
}