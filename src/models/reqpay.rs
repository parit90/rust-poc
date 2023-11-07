// src/models.rs

use serde::{Deserialize, Serialize};
//import module
pub mod payees;
pub mod payers;
pub mod respAuth;

//import public struct
pub use payees::Payees;
pub use payers::Payer;
use reqwest::Body;

// Testing purpose
// #[derive(Deserialize, Debug)]
// pub struct UserData {
//     name: String,
//     email: String,
// }

/**
 * ==========================================================
 * ReqPay XML Payload Starts Here
 * ==========================================================
 */

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ReqPay {
    #[serde(rename = "Head")]
    pub head: Head,
    #[serde(rename = "Meta")]
    meta: Meta,
    #[serde(rename = "Txn")]
    pub txn: Txn,
    #[serde(rename = "Payer")]
    pub payer: Payer,
    #[serde(rename = "Payees")]
    pub payee: Payees,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Head {
    ver: String,
    ts: String,
    orgId: String,
    msgId: String,
    prodType: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Tag {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Meta {
    #[serde(rename = "Tag")]
    tags: Vec<Tag>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Score {
    provider: String,
    #[serde(rename = "type")]
    score_type: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct RiskScore {
    #[serde(rename = "Score")]
    scores: Vec<Score>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Rule {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Rules {
    #[serde(rename = "Rule")]
    rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct QR {
    qVer: String,
    ts: String,
    qrMedium: String,
    expireTs: String,
    query: String,
    verToken: String,
    stan: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Txn {
    id: String,
    note: String,
    custRef: String,
    refId: String,
    refUrl: String,
    ts: String,
    refCategory: String,
    #[serde(rename = "type")]
    tx_type: String,
    RiskScore: RiskScore,
    Rules: Rules,
    QR: QR,
}

impl Into<Body> for ReqPay {
    fn into(self) -> Body {
        let xml = quick_xml::se::to_string(&self).unwrap();
        Body::from(xml)
    }
}
