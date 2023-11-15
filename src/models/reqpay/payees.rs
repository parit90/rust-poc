use serde::{Deserialize, Serialize};

//===========================Payee=======================

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Payees {
    #[serde(rename = "Payee")]
    pub Payee: Vec<Payee>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Payee {
    pub addr: String,
    pub name: String,
    pub seqNum: String,
    #[serde(rename = "type")]
    pub payee_type: String,
    pub code: String,
    #[serde(rename = "Institution")]
    institution: Institution,
    #[serde(rename = "Merchant")]
    merchant: Merchant,
    #[serde(rename = "Info")]
    info: Info,
    #[serde(rename = "Device")]
    device: Device,
    #[serde(rename = "Ac")]
    pub ac: Vec<Ac>,
    #[serde(rename = "Consent")]
    consent: Consent,
    #[serde(rename = "Amount")]
    pub amount: Amount,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Institution {
    QrPayLoad: String,
    conCode: String,
    netInstId: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Merchant {
    identifier: Identifier,
    Ownership: Ownership,
    Invoice: Invoice,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Identifier {
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
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Ownership {
    #[serde(rename = "type")]
    ownership_type: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Invoice {
    name: String,
    num: String,
    date: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Info {
    Identity: Identity,
    Rating: Rating,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Identity {
    id: String,
    #[serde(rename = "type")]
    identity_type: String,
    verifiedName: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Rating {
    VerifiedAddress: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Device {
    Tag: Vec<Tag>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Tag {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Ac {
    #[serde(rename = "addrType")]
    pub addr_type: String,
    #[serde(rename = "Detail")]
    pub detail: Vec<Detail>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Detail {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Consent {
    name: String,
    #[serde(rename = "type")]
    consent_type: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Amount {
    pub value: String,
    pub curr: String,
    #[serde(rename = "Split")]
    pub split: Vec<Split>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Split {
    name: String,
    value: String,
}
