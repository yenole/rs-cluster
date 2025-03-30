use axum::{
    extract::FromRef,
    http::{HeaderValue, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    cfg: Config,
}

impl From<Config> for AppState {
    fn from(input: Config) -> Self {
        Self { cfg: input }
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(input: &AppState) -> Self {
        input.cfg.clone()
    }
}

#[derive(Debug)]
pub struct RspAny<T>(pub T)
where
    T: Serialize;

impl<T> IntoResponse for RspAny<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let plant = match serde_json::to_string(&self.0) {
            Ok(json) => format!("{{\"code\":200,\"data\":{}}}", json),
            Err(e) => format!("{{\"code\":500,\"msg\":\"{}\"}}", e.to_string()),
        };
        (
            [(CONTENT_TYPE, HeaderValue::from_static("application/json"))],
            plant,
        )
            .into_response()
    }
}

impl IntoResponse for crate::Error {
    fn into_response(self) -> Response {
        (
            [(CONTENT_TYPE, HeaderValue::from_static("application/json"))],
            format!("{{\"code\":500,\"msg\":\"{}\"}}", self),
        )
            .into_response()
    }
}
