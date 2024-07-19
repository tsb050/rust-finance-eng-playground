use axum::{routing::get, Router};
use reqwest::Error;
use std::thread;
use futures::executor::block_on;


mod api_helpers;
use api_helpers::ApiResponse;

async fn get_request(fruit: &str) -> Result<ApiResponse, Error> {
    let response = reqwest::get(format!("https://www.fruityvice.com/api/fruit/{}", fruit)).await.unwrap();
    println!("Status: {}", response.status());

    let body: ApiResponse = response.json().await.unwrap();
    Ok(body)
}

// Assuming get_request() is in scope or can be injected
async fn handle_request() -> String {
    let fruits = vec!["apple", "banana", "mango"];
    let mut handles = vec![];

    for fruit in fruits.into_iter() {
        let handle = thread::spawn(move || {
            block_on(get_request(fruit))          
        });
        handles.push(handle);
    }

    let mut response_vec  = vec![];
    for handle in handles {
        let value = format!("{:?}", handle.join().unwrap().unwrap());
        response_vec.push(value);
    }

    format!("{:?}", response_vec)
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
