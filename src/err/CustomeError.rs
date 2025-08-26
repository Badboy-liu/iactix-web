use actix_web::{error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use derive_more::{Display, Error};

#[derive(Debug,Display,Error)]
pub enum MyError {
    #[display("InternalError")]
    InternalError,

    #[display("bad request")]
    BadClientData,


    #[display("timeout")]
    TimeOut,
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::InternalError => {StatusCode::INTERNAL_SERVER_ERROR}
            MyError::BadClientData => {StatusCode::BAD_REQUEST}
            MyError::TimeOut => {StatusCode::REQUEST_TIMEOUT}
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html()).body(self.to_string())
    }

}