use crate::domain::dto::ticket::{TicketCreateRequest, TicketUpdateRequest};
use crate::services::ticket_service;
use crate::AppState;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};

#[get("")]
async fn find_all_tickets(state: web::Data<AppState>) -> impl Responder {
    match ticket_service::find_all(&state.db_conn).await {
        Ok(tickets) => HttpResponse::Ok().json(tickets),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{ticket_id}")]
async fn find_one_ticket(
    state: web::Data<AppState>,
    ticket_id: web::Path<String>,
) -> impl Responder {
    match ticket_id.parse::<i32>() {
        Ok(ticket_id) => match ticket_service::find_one(ticket_id, &state.db_conn).await {
            Ok(ticket_option) => match ticket_option {
                Some(ticket) => HttpResponse::Ok().json(ticket),
                None => HttpResponse::NotFound().body("해당 ticket을 찾을 수 없습니다"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
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
    ticket_id: web::Path<String>,
    new_ticket: web::Json<TicketUpdateRequest>,
) -> impl Responder {
    match ticket_id.parse::<i32>() {
        Ok(ticket_id) => {
            match ticket_service::update(&state.db_conn, ticket_id, new_ticket.into_inner())
                .await
            {
                Ok(ticket_option) => match ticket_option {
                    Some(ticket) => HttpResponse::Ok().json(ticket),
                    None => HttpResponse::NotFound().body("ticket 수정 중 오류가 발생했습니다"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
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
        .service(find_all_tickets)
        .service(find_one_ticket)
        .service(add_ticket)
        .service(update_ticket)
        .service(delete_ticket)
}