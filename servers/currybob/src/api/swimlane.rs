use crate::entity::swimlane;
use crate::services::swimlane_service;
use crate::AppState;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};

#[get("/")]
async fn find_all_swimlanes(state: web::Data<AppState>) -> impl Responder {
    match swimlane_service::find_all(&state.db_conn).await {
        Ok(swimlanes) => HttpResponse::Ok().json(swimlanes),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{swimlane_id}")]
async fn find_one_swimlane(
    state: web::Data<AppState>,
    swimlane_id: web::Path<String>,
) -> impl Responder {
    match swimlane_id.parse::<i32>() {
        Ok(swimlane_id) => match swimlane_service::find_one(swimlane_id, &state.db_conn).await {
            Ok(swimlane_option) => match swimlane_option {
                Some(swimlane) => HttpResponse::Ok().json(swimlane),
                None => HttpResponse::NotFound().body("Swimlane not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/{new_name}")]
async fn add_swimlane(state: web::Data<AppState>, new_name: web::Path<String>) -> impl Responder {
    match swimlane_service::create(new_name.to_string(), &state.db_conn).await {
        Ok(swimlane) => HttpResponse::Ok().json(swimlane),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/{swimlane_id}")]
async fn update_swimlane(
    state: web::Data<AppState>,
    swimlane_id: web::Path<String>,
    new_swimlane: web::Json<swimlane::UpdateModel>,
) -> impl Responder {
    match swimlane_id.parse::<i32>() {
        Ok(swimlane_id) => {
            match swimlane_service::update(&state.db_conn, swimlane_id, new_swimlane.into_inner())
                .await
            {
                Ok(swimlane_option) => match swimlane_option {
                    Some(swimlane) => HttpResponse::Ok().json(swimlane),
                    None => HttpResponse::NotFound().body("Swimlane not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[delete("/{swimlane_id}")]
async fn delete_swimlane(state: web::Data<AppState>, swimlane_id: web::Path<String>) -> impl Responder {
    match swimlane_id.parse::<i32>() {
        Ok(swimlane_id) => match swimlane_service::delete(swimlane_id, &state.db_conn).await {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                }
                None => HttpResponse::Ok().body("No swimlane deleted"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

pub fn swimlanes_api() -> Scope {
    web::scope("/swimlanes")
        .service(find_all_swimlanes)
        .service(find_one_swimlane)
        .service(add_swimlane)
        .service(update_swimlane)
        .service(delete_swimlane)
}