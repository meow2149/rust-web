use axum::{Router, routing::get};

use crate::state::AppState;

use super::handler::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}
