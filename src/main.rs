use axum::{routing::get, Router};
use reqwest::Error;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct ApiResponse {
    name: String,
    family: String,
    id: i32,
    order: String,
}

async fn get_request() -> Result<ApiResponse, Error> {
    let response = reqwest::get("https://www.fruityvice.com/api/fruit/apple").await.unwrap();
    println!("Status: {}", response.status());

    let body: ApiResponse = response.json().await.unwrap();
    Ok(body)
}

// Assuming get_request() is in scope or can be injected
async fn handle_request() -> String {
    let result: ApiResponse = get_request().await.unwrap(); // Fetch data
    format!("{:?}", result) // Format as JSON (replace with desired format)
  }

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle_request));
    println!("Running on localhost !");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


//#[tokio::main]
//async fn main() {
//    let result: ApiResponse = get_request().await.unwrap();
//    println!("{:?}", result)/
//}