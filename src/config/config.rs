// src/config.rs
use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref APP_CONFIG: AppConfig = AppConfig::load();
}

pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub credit_req: String,
    pub debit_req: String,
    pub resp_pay: String,
    pub req_tx_confirm: String,
    pub validate_psp: String,
    pub enable_signature: bool,
    pub enable_kafka: bool,
    pub enable_dynamic_payload : bool,
    pub kafka_brokers: String,


}

impl AppConfig {
    pub fn load() -> AppConfig {
        AppConfig {
            // BASIC CONFIGURATION
            host: env::var("HOST").unwrap_or_else(|_| String::from("0.0.0.0")),
            port: env::var("PORT").unwrap_or_else(|_| String::from("8080")),

            // Bank and PSP URLs
            credit_req: env::var("CREDIT_REQ").unwrap_or_else(|_| String::from("default_value")),
            debit_req: env::var("DEBIT_REQ").unwrap_or_else(|_| String::from("default_value")),
            resp_pay: env::var("RESP_PAY").unwrap_or_else(|_| String::from("default_value")),
            req_tx_confirm: env::var("REQ_TX_CONFIRM").unwrap_or_else(|_| String::from("default_value")),
            validate_psp: env::var("VALIDATE_PSP").unwrap_or_else(|_| String::from("default_value")),


            // ENABLE DISABLE SOME FEATURES
            enable_signature: env::var("ENABLE_SIGNATURE").unwrap_or_else(|_| String::from("false")).parse().unwrap_or(false),
            enable_kafka: env::var("ENABLE_KAFKA").unwrap_or_else(|_| String::from("false")).parse().unwrap_or(false),
            enable_dynamic_payload: env::var("ENABLE_DYNAMIC_PAYLOAD").unwrap_or_else(|_| String::from("false")).parse().unwrap_or(false),

            // KAFKA BROKERS
            kafka_brokers: env::var("KAFKA_BROKERS").unwrap_or_else(|_| String::from("0.0.0.0:9002")),
        }
    }
}