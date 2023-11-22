// src/models.rs

use serde::{Deserialize, Serialize};
//import module
// mod payees;
// mod payers;
// pub mod respAuth;

//import public struct
pub use super::payees::Payees;
pub use super::payers::Payer;
use reqwest::Body;


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
    pub ver: String,
    pub ts: String,
    pub orgId: String,
    pub msgId: String,
    pub prodType: String,
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
pub struct RiskScore {
    #[serde(rename = "Score")]
    scores: Vec<Score>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Rule {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Rules {
    #[serde(rename = "Rule")]
    rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct QR {
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
    pub id: String,
    pub note: String,
    pub custRef: String,
    pub refId: String,
    pub refUrl: String,
    pub ts: String,
    pub refCategory: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub RiskScore: RiskScore,
    pub Rules: Rules,
    pub QR: QR,
}

impl Into<Body> for ReqPay {
    fn into(self) -> Body {
        let xml = quick_xml::se::to_string(&self).unwrap();
        Body::from(xml)
    }
}
