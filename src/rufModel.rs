// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserData {
    name: String,
    email: String,
}


/**
 * ==========================================================
 * ReqPay XML Payload Starts Here
 * ==========================================================
 */

#[derive(Debug, Deserialize)]
pub struct ReqPay {
    #[serde(rename = "Head")]
    head: Head,
    #[serde(rename = "Meta")]
    meta: Meta,
    #[serde(rename = "Txn")]
    txn: Txn,
    #[serde(rename = "Payer")]
    payer:Payer
}

#[derive(Debug, Deserialize)]
struct Head {
    ver: String,
    ts: String,
    orgId: String,
    msgId: String,
    prodType: String,
}

#[derive(Debug, Deserialize)]
struct Tag {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct Meta {
    #[serde(rename = "Tag")]
    tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
struct Score {
    provider: String,
    #[serde(rename = "type")]
    score_type: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct RiskScore {
    #[serde(rename = "Score")]
    scores: Vec<Score>,
}

#[derive(Debug, Deserialize)]
struct Rule {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct Rules {
    #[serde(rename = "Rule")]
    rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
struct QR {
    qVer: String,
    ts: String,
    qrMedium: String,
    expireTs: String,
    query: String,
    verToken: String,
    stan: String,
}

#[derive(Debug, Deserialize)]
struct Txn {
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


//======================================Payer=============================

#[derive(Debug, Deserialize)]
struct Merchant {
    identifier: Identifier,
    Ownership: Ownership,
    Invoice: Invoice,
}

#[derive(Debug, Deserialize)]
struct Identifier {
    subCode: String,
    mid: String,
    sid: String,
    tid: String,
    merchantType: String,
    merchantGenre: String,
    onBoardingType: String,
    pinCode: String,
    regIdNo: String,
    tier: String,
    merchantLoc: String,
    merchantInstId: String,
}

#[derive(Debug, Deserialize)]
struct Ownership {
    #[serde(rename = "type")]
    ownership_type: String,
}

#[derive(Debug, Deserialize)]
struct Invoice {
    name: String,
    num: String,
    date: String,
}

#[derive(Debug, Deserialize)]
struct Institution {
    #[serde(rename = "type")]
    institution_type: String,
    route: String,
    Name: Name,
    Purpose: Purpose,
    Originator: Originator,
    Beneficiary: Beneficiary,
}

#[derive(Debug, Deserialize)]
struct Name {
    value: String,
    acNum: String,
}

#[derive(Debug, Deserialize)]
struct Purpose {
    code: String,
    note: String,
}

#[derive(Debug, Deserialize)]
struct Originator {
    name: String,
    #[serde(rename = "type")]
    originator_type: String,
    refNo: String,
    Address: Address,
}

#[derive(Debug, Deserialize)]
struct Address {
    location: String,
    city: String,
    country: String,
    geocode: String,
}

#[derive(Debug, Deserialize)]
struct Beneficiary {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Info {
    Identity: Identity,
    Rating: Rating,
}

#[derive(Debug, Deserialize)]
struct Identity {
    id: String,
    #[serde(rename = "type")]
    identity_type: String,
    verifiedName: String,
}

#[derive(Debug, Deserialize)]
struct Rating {
    VerifiedAddress: String,
}

#[derive(Debug, Deserialize)]
struct Device {
    Tag: Vec<PayerTag>,
}

#[derive(Debug, Deserialize)]
struct PayerTag {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct Ac {
    #[serde(rename = "addrType")]
    addr_type: String,
    Detail: Vec<Detail>,
}

#[derive(Debug, Deserialize)]
struct Detail {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct Consent {
    name: String,
    #[serde(rename = "type")]
    consent_type: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct Creds {
    Cred: Vec<Cred>,
}

#[derive(Debug, Deserialize)]
struct Cred {
    #[serde(rename = "type")]
    cred_type: String,
    subType: String,
    #[serde(rename = "MetaPyr")]
    Meta: MetaPyr,
    Data: String,
}

#[derive(Debug, Deserialize)]
struct MetaPyr {
    lk: String,
    ac: String,
    sa: String,
    uid: String,
    ver: String,
}

#[derive(Debug, Deserialize)]
struct Amount {
    value: String,
    curr: String,
    Split: Vec<Split>,
}

#[derive(Debug, Deserialize)]
struct Split {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct Payer {
    addr: String,
    name: String,
    seqNum: String,
    #[serde(rename = "type")]
    payer_type: String,
    code: String,
    Merchant: Merchant,
    Institution: Institution,
    Info: Info,
    Device: Device,
    Ac: Vec<Ac>,
    Consent: Consent,
    Creds: Creds,
    Amount: Amount,
}
