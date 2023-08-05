use actix_web::{get, web, Error as ActixError, Responder, Result as ActixResult, Scope};

use crate::AppState;

#[get("/")]
async fn get_tickets(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let tickets = state.ticket_repository.get_tickets().await;
    Ok(web::Json(tickets))
}

pub fn tickets_api() -> Scope {
    web::scope("/tickets").service(get_tickets)
}