use actix_web::{error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::http::StatusCode;
use derive_more::{Display, Error};
use crate::err::CustomeError::MyError;

#[derive(Debug,Display,Error)]
pub enum UserError{
    #[display("An internal error has occurred.")]
    InternalError,
}

impl error::ResponseError for UserError{
    fn status_code(&self) -> StatusCode {
        match *self { UserError::InternalError => {StatusCode::INTERNAL_SERVER_ERROR}, _ =>StatusCode::BAD_REQUEST }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header((CONTENT_TYPE, "application/json"))
            .body(self.to_string())
    }
}