use serde::{Deserialize, Serialize};

//======================================Payer=============================

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Merchant {
    identifier: Identifier,
    Ownership: Ownership,
    Invoice: Invoice,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
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

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Ownership {
    #[serde(rename = "type")]
    ownership_type: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Invoice {
    name: String,
    num: String,
    date: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Institution {
    #[serde(rename = "type")]
    institution_type: String,
    route: String,
    Name: Name,
    Purpose: Purpose,
    Originator: Originator,
    Beneficiary: Beneficiary,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Name {
    value: String,
    acNum: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Purpose {
    code: String,
    note: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Originator {
    name: String,
    #[serde(rename = "type")]
    originator_type: String,
    refNo: String,
    Address: Address,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Address {
    location: String,
    city: String,
    country: String,
    geocode: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Beneficiary {
    name: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Info {
    Identity: Identity,
    Rating: Rating,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Identity {
    id: String,
    #[serde(rename = "type")]
    identity_type: String,
    verifiedName: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Rating {
    VerifiedAddress: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Device {
    Tag: Vec<PayerTag>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct PayerTag {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Ac {
    #[serde(rename = "addrType")]
    addr_type: String,
    Detail: Vec<Detail>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Detail {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Consent {
    name: String,
    #[serde(rename = "type")]
    consent_type: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Creds {
    Cred: Vec<Cred>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Cred {
    #[serde(rename = "type")]
    cred_type: String,
    subType: String,
    // #[serde(rename = "MetaPyr")]
    // MetaPyr: MetaP,
    Data: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct MetaP {
    lk: String,
    ac: String,
    sa: String,
    uid: String,
    ver: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Amount {
    value: String,
    curr: String,
    Split: Vec<Split>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Split {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Payer {
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
