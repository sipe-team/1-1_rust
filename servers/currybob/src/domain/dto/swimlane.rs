use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SwimlaneCreateRequest {
    pub board_id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct SwimlaneUpdateRequest {
    pub board_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
}