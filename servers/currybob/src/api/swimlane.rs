use actix_web::{get, web, Error as ActixError, Responder, Result as ActixResult, Scope};

use crate::AppState;

#[get("/")]
async fn get_swimlanes(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let swimlanes = state.swimlane_repository.get_swimlanes().await;
    Ok(web::Json(swimlanes))
}

pub fn swimlanes_api() -> Scope {
    web::scope("/swimlanes").service(get_swimlanes)
}