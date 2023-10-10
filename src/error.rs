use std::fmt::Display;

use axum::{response::IntoResponse, http::StatusCode};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail, 

    // -- Model errors.
    TicketDeleteFailIdNotFound { id: u64 },
}

/// For more details: https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html#implementing-intoresponse
/// Usually you wouldn't need to implement `IntoResponse` manually, but it can be necessary if we 
/// have a custom error type
/// 
/// NEVER PASS YOUR SERVER ERROR TO THE CLIENT
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let body = match self {
           Error::LoginFail => "FAILED_TO_LOGIN" ,
           _ => "UNHANDLED_CLIENT_ERROR"
        };
        
        // you can implement traits for tuples
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

// region: --- Error boilerplate
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

// end region: --- Error boilerplate