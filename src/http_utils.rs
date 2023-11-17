use once_cell::sync::Lazy;
use reqwest::{Client, Method};

use actix_web::Result;

use crate::custom_error::CustomError;


// Static client instance, initialized once
static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub async fn make_request(url: &str, method: &str, body: &str) -> Result<String, CustomError> {

    let reqwest_method = match Method::from_bytes(method.as_bytes()) {
        Ok(m) => m,
        Err(_) => {
            
            let error_string: String = String::from("Invalid HTTP method");
            // If it's not a valid method, return an error
            return Err(CustomError(error_string));
        }
    };

    // Make a request using the static client
    let response = CLIENT
        .request(reqwest_method, url)
        .header("Content-Type", "application/xml")
        .body(body.to_owned())
        .send()
        .await?;

    // Check if the response status is successful (2xx)
    if response.status().is_success() {
        // Read the response body as a string
        let body = response.text().await?;
        Ok(body)
    } else {
        // Handle non-successful responses (e.g., log or return an error)
        let status_code = response.status();
        let error_message = format!("Request failed with status code: {}", status_code);
        let response_body = response.text().await.unwrap_or_default();
        let error_description = format!("{}\nResponse Body: {}", error_message, response_body);

        return Err(CustomError(error_description) );
    }
}


