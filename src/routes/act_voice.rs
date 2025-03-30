use axum::{Json, Router, extract::State, routing::post};
use payload::{ReqAsr, RspAsr};
use tokio::fs;

use super::{Result, state::AppState};
use crate::{Ok, asr, config::Config};

pub fn routes() -> Router<AppState> {
    Router::new().route("/asr", post(handle_asr))
}

mod payload {
    use serde::Deserialize;

    use crate::module;

    #[derive(Debug, Clone, Deserialize)]
    pub struct ReqAsr {
        // #[serde(default)]
        // pub auth: String,
        #[serde(default)]
        pub data: String,
    }

    pub type RspAsr = module::AsrResult;
}

async fn handle_asr(State(cfg): State<Config>, Json(mut v): Json<ReqAsr>) -> Result<RspAsr> {
    // TODO 过滤TOKEN
    if cfg!(debug_assertions) && v.data.is_empty() {
        v.data = fs::read_to_string("/mnt/d/1.txt").await.unwrap();
    }
    if v.data.is_empty() {
        return Err("data is empty".into());
    }
    Ok!(asr(&cfg.asr_secret_id, &cfg.asr_secret_key, &v.data).await?)
}
