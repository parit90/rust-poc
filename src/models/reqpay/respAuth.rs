use serde::{Deserialize, Serialize};

//===========================Payee=======================

#[derive(Deserialize, Debug, Serialize)]
pub struct RespAuthDetail {
    pub Name: String,
}
