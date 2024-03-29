use axum::{
  body::Body,
  response::{IntoResponse, Response},
};
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
  #[error("Something awful just happened: `{0}`.")]
  Any(#[from] anyhow::Error),
  #[error("Json Error: `{0}`.")]
  Json(#[from] serde_json::Error),
  #[error("Jq Error: `{0}`.")]
  JqError(#[from] crate::jq::JqError),
  #[error("Parsing Error: `{0}`.")]
  ParsingError(#[from] crate::parsing::ParsingError),
  #[error("Reqwest Error: `{0}`.")]
  ReqwestError(#[from] reqwest::Error),
}

impl IntoResponse for HttpError {
  fn into_response(self) -> Response {
    let mut response = Response::new(Body::from(self.to_string()));
    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    response
  }
}
