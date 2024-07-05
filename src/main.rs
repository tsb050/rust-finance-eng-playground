use axum::{routing::get, Router};
use reqwest::Error;

async fn get_request() -> Result<(), Error> {
    let response = reqwest::get("https://www.fruityvice.com/api/fruit/apple").await.unwrap();
    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}


#[tokio::main]
async fn serve_host() {
    let app = Router::new().route("/", get(|| async { "Rust launched" }));
    println!("Running on localhost !");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    get_request().await?;
    Ok(())
}