use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    name: String,
    family: String,
    id: i32,
    order: String,
}