use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;

use crate::board::application::error::board_error::BoardPortError;
use crate::common::routes::model::ErrorResponse;

impl ResponseError for BoardPortError {
    fn status_code(&self) -> StatusCode {
        match self {
            BoardPortError::NotFound(_) => StatusCode::NOT_FOUND,
            BoardPortError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let message = match self {
            BoardPortError::NotFound(id) => format!("board {} not found.", id),
            BoardPortError::UnexpectedError(_) => String::from("Internal Server Error"),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorResponse::from(message))
    }
}