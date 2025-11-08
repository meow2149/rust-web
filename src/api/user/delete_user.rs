use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::EntityTrait;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

use super::entity;

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<StatusCode> {
    let res = entity::Entity::delete_by_id(id).exec(state.db()).await?;

    if res.rows_affected == 0 {
        return Err(AppError::not_found(format!("user {id} not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}
