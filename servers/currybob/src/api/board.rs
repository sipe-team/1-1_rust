use actix_web::{get, web, Error as ActixError, Responder, Result as ActixResult, Scope};

use crate::AppState;

#[get("/")]
async fn get_boards(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let boards = state.board_repository.get_boards().await;
    Ok(web::Json(boards))
}

pub fn boards_api() -> Scope {
    web::scope("/boards").service(get_boards)
}