use tokio::sync::mpsc;
use tokio::task;
use std::error::Error;
use axum::{routing::get, Router};

mod api_helpers;
use api_helpers::ApiResponse;

async fn get_request(fruit: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(format!("https://www.fruityvice.com/api/fruit/{}", fruit)).await.unwrap();
    println!("Status: {}", response.status());

    let body: ApiResponse = response.json().await.unwrap();
    Ok(format!("{:?}", body))
}

async fn handle_request22() -> Result<String, Box<dyn Error>> {
    let fruits = vec!["apple", "banana", "mango"];
    let (tx, mut rx) = mpsc::channel(fruits.len());

    // Spawn tasks for each fruit, sending results through the channel
    for fruit in fruits.clone() {
        let tx_clone = tx.clone();
        task::spawn(async move {
            let result = get_request(fruit).await.unwrap();
            tx_clone.send(result).await.unwrap();
        });
    }

    // Collect results from tasks
    let mut response_vec = Vec::new();
    for _ in 0..fruits.len() {
        response_vec.push(rx.recv().await.unwrap());
    }

    Ok(format!("{:?}", response_vec))
}

async fn serve_results() -> String {
    match handle_request22().await {
        Ok(response) => format!("{:?}", response),
        Err(e) => format!("Error: {}", e),
    }
}


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(serve_results));
    println!("Running on localhost !");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


