// src/models.rs

use serde::{Deserialize, Serialize};
pub mod reqpay;
pub mod reqAuthDetails;

pub use reqpay::respAuth;
pub use reqpay::Payees;
pub use reqpay::Payer;
pub use reqpay::ReqPay;
pub use reqAuthDetails::ReqAuthDetails;

#[derive(Deserialize, Debug)]
pub struct UserData {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AuthResponse {
    auth_response: bool,
    status: i16,
    validation: String,
}
