use crate::models;

pub fn validate_req_pay(req_pay: &models::ReqPay) -> Result<(), &'static str> {
    // println!("validate_req_pay");
    if !is_valid_addr(&req_pay.payee.Payee[0].addr) {
        return Err("Invalid name");
    }
    if !is_valid_code(&req_pay.payee.Payee[0].code) {
        return Err("Invalid code");
    }
    if !is_valid_amount(&req_pay.payee.Payee[0].amount.value) {
        return Err("Invalid amount");
    }
    if !is_valid_aadhar(&req_pay.payee.Payee[0].ac[0].detail[0].value) {
        return Err("Invalid aadhar");
    }
    Ok(())
}
fn is_valid_addr(name: &str) -> bool{
    name.contains('@')
}

fn is_valid_code(code: &str) -> bool{
    code.chars().all(char::is_alphanumeric)
}

fn is_valid_amount(amount: &str) -> bool {
    if let Ok(parsed_amount) = amount.parse::<f64>() {
        return parsed_amount > 0.0;
    }
    false
}

fn is_valid_aadhar(details: &str) -> bool {
    details.chars().all(char::is_alphanumeric)
}