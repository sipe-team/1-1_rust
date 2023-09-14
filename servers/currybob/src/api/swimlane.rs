use crate::domain::dto::swimlane::{SwimlaneCreateRequest, SwimlaneUpdateRequest};
use crate::domain::dto::ticket::TicketSortQuery;
use crate::services::{swimlane_service, ticket_service};
use crate::AppState;

use actix_web::web::Query;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};

#[get("")]
async fn find_all(state: web::Data<AppState>) -> impl Responder {
    match swimlane_service::find_all(&state.db_conn).await {
        Ok(swimlanes) => HttpResponse::Ok().json(swimlanes),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{swimlane_id}")]
async fn find_one(
    state: web::Data<AppState>,
    swimlane_id: web::Path<i32>,
) -> impl Responder {
    let swimlane_id = swimlane_id.into_inner();

    match swimlane_service::find_one(swimlane_id, &state.db_conn).await {
        Ok(exist) => match exist {
            Some(swimlane) => HttpResponse::Ok().json(swimlane),
            None => HttpResponse::NotFound().body("해당 swimlane을 찾을 수 없습니다"),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{swimlane_id}/tickets")]
async fn find_all_tickets_by_id(
    state: web::Data<AppState>,
    swimlane_id: web::Path<i32>,
    query: Query<TicketSortQuery>
) -> impl Responder {
    let swimlane_id = swimlane_id.into_inner();

    match ticket_service::find_all_by_swimlane_id(swimlane_id, &state.db_conn, &query).await {
        Ok(tickets) => HttpResponse::Ok().json(tickets),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("")]
async fn add_swimlane(
    state: web::Data<AppState>,
    data: web::Json<SwimlaneCreateRequest>
) -> impl Responder {
    match swimlane_service::create(data.into_inner(), &state.db_conn).await {
        Ok(swimlane) => HttpResponse::Ok().json(swimlane),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/{swimlane_id}")]
async fn update_swimlane(
    state: web::Data<AppState>,
    swimlane_id: web::Path<i32>,
    payload: web::Json<SwimlaneUpdateRequest>,
) -> impl Responder {
    let swimlane_id = swimlane_id.into_inner();
    let payload = payload.into_inner();

    match swimlane_service::update(&state.db_conn, swimlane_id, payload).await {
        Ok(swimlane) => HttpResponse::Ok().json(swimlane),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/{swimlane_id}")]
async fn delete_swimlane(state: web::Data<AppState>, swimlane_id: web::Path<String>) -> impl Responder {
    match swimlane_id.parse::<i32>() {
        Ok(swimlane_id) => match swimlane_service::delete(swimlane_id, &state.db_conn).await {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{}개의 row가 삭제되었습니다", result.rows_affected))
                }
                None => HttpResponse::Ok().body("swimlane이 삭제되지 않았습니다"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

pub fn swimlanes_api() -> Scope {
    web::scope("/swimlanes")
        .service(find_all)
        .service(find_all_tickets_by_id)
        .service(find_one)
        .service(add_swimlane)
        .service(update_swimlane)
        .service(delete_swimlane)
}