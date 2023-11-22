use serde::{Deserialize, Serialize};

//======================================Payer=============================

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Merchant {
    pub identifier: Identifier,
    pub Ownership: Ownership,
    pub Invoice: Invoice,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Identifier {
    pub subCode: String,
    pub mid: String,
    pub sid: String,
    pub tid: String,
    pub merchantType: String,
    pub merchantGenre: String,
    pub onBoardingType: String,
    pub pinCode: String,
    pub regIdNo: String,
    pub tier: String,
    pub merchantLoc: String,
    pub merchantInstId: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Ownership {
    #[serde(rename = "type")]
    pub ownership_type: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Invoice {
    pub name: String,
    pub num: String,
    pub date: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Institution {
    #[serde(rename = "type")]
    pub institution_type: String,
    pub route: String,
    pub Name: Name,
    pub Purpose: Purpose,
    pub Originator: Originator,
    pub Beneficiary: Beneficiary,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Name {
    pub value: String,
    pub acNum: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Purpose {
    pub code: String,
    pub note: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Originator {
    pub name: String,
    #[serde(rename = "type")]
    pub originator_type: String,
    pub refNo: String,
    pub Address: Address,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Address {
    pub location: String,
    pub city: String,
    pub country: String,
    pub geocode: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Beneficiary {
    pub name: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Info {
    pub Identity: Identity,
    pub Rating: Rating,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Identity {
    pub id: String,
    #[serde(rename = "type")]
    pub identity_type: String,
    pub verifiedName: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Rating {
    pub VerifiedAddress: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Device {
    pub Tag: Vec<PayerTag>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct PayerTag {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Ac {
    #[serde(rename = "addrType")]
    pub addr_type: String,
    pub Detail: Vec<Detail>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Detail {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Consent {
    pub name: String,
    #[serde(rename = "type")]
    pub consent_type: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Creds {
    pub Cred: Vec<Cred>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Cred {
    #[serde(rename = "type")]
    pub cred_type: String,
    pub subType: String,
    // #[serde(rename = "MetaPyr")]
    // MetaPyr: MetaP,
    pub Data: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct MetaP {
    pub lk: String,
    pub ac: String,
    pub sa: String,
    pub uid: String,
    pub ver: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Amount {
    pub value: String,
    pub curr: String,
    pub Split: Vec<Split>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Split {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Payer {
    pub addr: String,
    pub name: String,
    pub seqNum: String,
    #[serde(rename = "type")]
    pub payer_type: String,
    pub code: String,
    pub Merchant: Merchant,
    pub Institution: Institution,
    pub Info: Info,
    pub Device: Device,
    pub Ac: Vec<Ac>,
    pub Consent: Consent,
    pub Creds: Creds,
    pub Amount: Amount,
}
