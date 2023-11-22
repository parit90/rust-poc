
// TODO: create models for all payload
// TODO: Create Main and Sub Api's ...
// DONE: Serialize and Deserialize the req body
// DONE: common callback function 
// HOLD: Mark Success with kafka push or Some notes
// TODO: Set Proper logs , because production require a proper logs


use actix_web::{web, App, HttpServer};
mod handlers;
mod validation;
mod utils;
mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/reqpay", web::post().to(handlers::handle_reqpay))
            .route("/respauth/callback", web::post().to(handlers::handle_resp_auth_details))
            .route("/debitresp/callback", web::post().to(handlers::handle_debit_resp))
            .route("/creditresp/callback", web::post().to(handlers::handle_credit_resp))
            .route("/restxconfirm/callback", web::post().to(handlers::handle_resp_txn_confirm))
        
            // Add more routes as needed
    })

    .bind("192.168.68.147:8080")?
    .run()
    .await
    
}

