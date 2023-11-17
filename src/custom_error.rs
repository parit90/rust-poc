use std::fmt;


#[derive(Debug)]
pub struct CustomError(pub String);

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        // You can customize the conversion here
        CustomError(format!("Reqwest error: {}", error))
    }
}