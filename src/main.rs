use actix_web::{web, App, HttpResponse, HttpServer, Result, web::{Data, Buf}, middleware::{Logger}};
use kafka::create_kafka_producer;
use serde_xml_rs::{from_str, from_reader};
use models::ReqAuthDetails;
mod req_auth_details_creator;
use req_auth_details_creator::create_req_auth_details;

mod validate_request;
use validate_request::validate_req_pay;
// use quick_xml::se::to_string;
use reqwest::{Client, Error};
use std::fmt;
//use std::io;
use dotenv::dotenv;
use tokio::spawn;
use lazy_static::lazy_static;
use std::sync::Mutex;
use flame;
use std::fs::File;

mod models;
mod validate_psp;
mod debit_req;
mod credit_req;
mod resp_pay;
mod callback;
mod kafka;
mod sign;
extern crate num_cpus;

use rdkafka::{error::KafkaError, ClientContext};
use rdkafka::producer::FutureProducer;



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

async fn process_xml(data: web::Bytes, app_data: Data<MyURLs>, sanitation:Data<Sanitation>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    // Deserialize the XML data directly from a reader
    let reader = data.clone().reader();
    let req_pay: Result<models::ReqPay, serde_xml_rs::Error> = from_reader(reader);
    
    match req_pay {
        Ok(req_pay) => {
            // Clone the data for validation tasks
            // let req_pay_clone = req_pay.clone();
            // let app_data_clone = app_data.clone();

            // if !is_valid_addr(&req_pay.payee.Payee[0].addr){
            //     return Ok(HttpResponse::BadRequest().body("Invalid name"));
            // }
            // if !is_valid_code(&req_pay.payee.Payee[0].code){
            //     return Ok(HttpResponse::BadRequest().body("Invalid code"));
            // }
            
            // if !is_valid_amount(&req_pay.payee.Payee[0].amount.value){
            //     return Ok(HttpResponse::BadRequest().body("Invalid amount"));
            // }

            // if &req_pay.payee.Payee[0].ac[0].addr_type == "AADHAR" {
            //     if !is_valid_aadhar(&req_pay.payee.Payee[0].ac[0].detail[0].value){
            //         return Ok(HttpResponse::BadRequest().body("Invalid aadhar"));
            //     }
            // }

            if(sanitation.validation_switch) {
                if let Err(error_msg) = validate_request::validate_req_pay(&req_pay) {
                    return Ok(HttpResponse::BadRequest().body(error_msg));
                }
            }
            let signature = sign::get_signature( data , sanitation.enable_signature).await;
            // println!("====={:?}",signature);
            if(sanitation.payload_transform_switch){
                let _reqAuthDetails = create_req_auth_details(&req_pay);  
            }

            // Spawn the validation task
            let validate_task = spawn(async move {
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
    sanitation:Data<Sanitation>
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

            if sanitation.use_kafka {
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
            else {
                Ok(HttpResponse::Ok().body("Success"))
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

pub struct Sanitation {
    signature:Option<String>,
    use_kafka:bool,
    enable_signature : bool,
    validation_switch:bool,
    payload_transform_switch:bool
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    ////println!("Number of CPU threads: {}", num_threads);

    let CREDIT_REQ_URL = std::env::var("CREDIT_REQ").unwrap_or_default(); 
    let DEBIT_REQ_URL = std::env::var("DEBIT_REQ").unwrap_or_default();
    let RESP_PAY_URL = std::env::var("RESP_PAY").unwrap_or_default();
    let REQ_TX_CONFIRM_URL = std::env::var("REQ_TX_CONFIRM").unwrap_or_default();
    let VALIDATE_PSP_URL = std::env::var("VALIDATE_PSP").unwrap_or_default();
    let ENABLE_SIGNATURE = std::env::var("ENABLE_SIGNATURE").unwrap_or_default().parse::<bool>().unwrap();
    let USE_KAFKA = std::env::var("USE_KAFKA").unwrap_or_default().parse::<bool>().unwrap();
    let VALIDATION_SWITCH = std::env::var("VALIDATION_SWITCH").unwrap_or_default().parse::<bool>().unwrap();
    let PAYLOAD_TRANSFORM_SWITCH = std::env::var("PAYLOAD_TRANSFORM_SWITCH").unwrap_or_default().parse::<bool>().unwrap();
    

    let MyURLs = Data::new(MyURLs { 
        CREDIT_REQ_URL,
        DEBIT_REQ_URL,
        RESP_PAY_URL,
        REQ_TX_CONFIRM_URL,
        VALIDATE_PSP_URL
    });

    let Sanitation = Data::new(
        Sanitation{
            signature:None,
            use_kafka:USE_KAFKA,
            enable_signature : ENABLE_SIGNATURE,
            validation_switch : VALIDATION_SWITCH,
            payload_transform_switch:PAYLOAD_TRANSFORM_SWITCH
        }
    );
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .app_data(Data::clone( &MyURLs))
        .app_data(Data::clone( &Sanitation))
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
    .bind("0.0.0.0:8083")?
    .run()
    .await?;
    // flame::end("main");

    // // Dump the Flamegraph data to a file.
    // let output_file = File::create("flamegraph-output.html").unwrap();
    // flame::dump_html(output_file).unwrap();
    
    Ok(())
}