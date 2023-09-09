use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BoardCreateRequest {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct BoardUpdateRequest {
    name: String,
}
