use actix_web::HttpResponse;

use crate::models::ReqPay;


pub async fn validate_parameters(req_pay: &ReqPay) -> Result<(), HttpResponse> {
    // Implement your validation logic here
    if req_pay.head.orgId.is_empty() {
        return Err(HttpResponse::BadRequest().body("Invalid org Id"));
    }

    if req_pay.head.ver.is_empty() {
        return Err(HttpResponse::BadRequest().body("Invalid TnxId"));
    }

    if req_pay.head.msgId.is_empty() {
        return Err(HttpResponse::BadRequest().body("Invalid org Id"));
    }

    if req_pay.txn.id.is_empty() {
        return Err(HttpResponse::BadRequest().body("Invalid TnxId"));
    }

    // Add more validation as needed

    Ok(())
}