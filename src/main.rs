use axum::{routing::get, Router};


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Rust launched" }));
    println!("Running on localhost !");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
