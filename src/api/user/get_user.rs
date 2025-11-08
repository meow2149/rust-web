use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use sea_orm::EntityTrait;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

use super::{dto::UserResponse, entity};

#[debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<UserResponse>> {
    let user = entity::Entity::find_by_id(id)
        .one(state.db())
        .await?
        .ok_or_else(|| AppError::not_found(format!("user {id} not found")))?;

    Ok(Json(UserResponse::from(user)))
}
