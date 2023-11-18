use once_cell::sync::Lazy;
use reqwest::{Client, Response, Result};
use std::error::Error;

// Static client instance, initialized once
static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub async fn make_request(url: &str, method: &str, body: &str) -> Result<String> {
    // Make a request using the static client
    let response = CLIENT
        .request(reqwest::Method::from_bytes(method.as_bytes())?, url)
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
        Err(reqwest::Error::custom(format!(
            "Request failed with status code: {}",
            response.status()
        )))
    }
}



/*** 
   let url = "https://example.com";
    let xml_body = r#"<your_xml_payload_here>"#;
    let result = http_utils::make_request(url, "POST", xml_body).await;

    match result {
        Ok(response) => {
            println!("Response: {}", response);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
 */