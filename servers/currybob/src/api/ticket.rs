use crate::domain::dto::ticket::{TicketCreateRequest, TicketUpdateRequest, TicketSortQuery};
use crate::services::ticket_service;
use crate::AppState;

use actix_web::web::Query;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};

#[get("")]
async fn find_all(
    state: web::Data<AppState>,
    query: Query<TicketSortQuery>
) -> impl Responder {
    match ticket_service::find_all(&state.db_conn, &query).await {
        Ok(tickets) => HttpResponse::Ok().json(tickets),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{ticket_id}")]
async fn find_one(
    state: web::Data<AppState>,
    ticket_id: web::Path<i32>,
) -> impl Responder {
    let ticket_id = ticket_id.into_inner();

    match ticket_service::find_one(ticket_id, &state.db_conn).await {
        Ok(ticket_option) => match ticket_option {
            Some(ticket) => HttpResponse::Ok().json(ticket),
            None => HttpResponse::NotFound().body("해당 ticket을 찾을 수 없습니다"),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("")]
async fn add_ticket(
    state: web::Data<AppState>,
    data: web::Json<TicketCreateRequest>,
) -> impl Responder {
    match ticket_service::create(data.into_inner(), &state.db_conn).await {
        Ok(ticket) => HttpResponse::Ok().json(ticket),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/{ticket_id}")]
async fn update_ticket(
    state: web::Data<AppState>,
    ticket_id: web::Path<i32>,
    payload: web::Json<TicketUpdateRequest>,
) -> impl Responder {
    let ticket_id = ticket_id.into_inner();
    let payload = payload.into_inner();

    match ticket_service::update(&state.db_conn, ticket_id, payload).await {
        Ok(ticket) => HttpResponse::Ok().json(ticket),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/{ticket_id}")]
async fn delete_ticket(state: web::Data<AppState>, ticket_id: web::Path<String>) -> impl Responder {
    match ticket_id.parse::<i32>() {
        Ok(ticket_id) => match ticket_service::delete(ticket_id, &state.db_conn).await {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{}개의 row가 삭제되었습니다", result.rows_affected))
                }
                None => HttpResponse::Ok().body("ticket이 삭제되지 않았습니다"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

pub fn tickets_api() -> Scope {
    web::scope("/tickets")
        .service(find_all)
        .service(find_one)
        .service(add_ticket)
        .service(update_ticket)
        .service(delete_ticket)
}