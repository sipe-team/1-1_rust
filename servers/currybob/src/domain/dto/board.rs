use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BoardCreateRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct BoardUpdateRequest {
    pub name: String,
}