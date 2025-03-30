use axum::Router;

use super::{act_voice, state::AppState};

pub fn setup_routing(state: AppState) -> Router {
    Router::new()
        .nest("/v1", act_voice::routes())
        // .route_layer(from_fn_with_state(state.clone(), authorize))
        .with_state(state)
}
