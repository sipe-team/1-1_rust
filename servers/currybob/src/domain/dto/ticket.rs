use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TicketCreateRequest {
    pub swimlane_id: i32,
    pub name: String,
    pub priority: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TicketUpdateRequest {
    pub swimlane_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub priority: Option<i32>,
}

#[derive(Deserialize)]
pub struct TicketSortQuery {
    pub name: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub priority: Option<String>,
}