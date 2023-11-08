async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
        .service(
            web::resource("/respauth/callback")
            .route(web::post().to(resp_auth_callback))
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    
    Ok(())
}