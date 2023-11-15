// Use serde for serialization/deserialization if needed
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ReqAuthDetails {
    pub head: Head,
    pub txn: Txn,
    pub payees: Vec<Payee>,
    pub payer: Payer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Head {
    pub msg_id: String,
    pub org_id: String,
    pub ts: String,
    pub ver: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Txn {
    pub cust_ref: String,
    pub id: String,
 
    pub note: String,

    pub ref_id: String,
    pub ref_url: String,
    pub ts: String,
    #[serde(rename = "type")]
    pub txn_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payee {
    pub addr: String,
    pub code: String,
    pub name: String,
    pub seq_num: String,
    #[serde(rename = "type")]
    pub payee_type: String,
    pub amount: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub curr: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payer {
    pub addr: String,
    pub code: String,
    pub name: String,
    pub seq_num: String,
    #[serde(rename = "type")]
    pub payer_type: String,
    pub info: Info,
    pub ac: Vec<Ac>,
    pub amount: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub identity: Identity,
    pub rating: Rating,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    pub id: String,
    #[serde(rename = "type")]
    pub identity_type: String,
    pub verified_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    pub verified_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ac {
    pub addr_type: String,
    pub detail: Vec<Detail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
    pub name: String,
    pub value: String,
}
