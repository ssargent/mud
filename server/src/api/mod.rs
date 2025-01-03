use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

pub mod game;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload<T> {
    pub data: T,
}

pub enum ApiResponse<T> {
    Ok,
    NotFound,
    NotChanged,
    BadRequest(String),
    Created(Payload<T>),
    Error(String),
    JsonData(Payload<T>),
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => (StatusCode::OK).into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND).into_response(),
            Self::NotChanged => (StatusCode::NOT_MODIFIED).into_response(),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            Self::Created(data) => (StatusCode::CREATED, Json(data)).into_response(),
            Self::Error(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}
