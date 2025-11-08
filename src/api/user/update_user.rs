use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use time::OffsetDateTime;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

use super::{
    dto::{UpdateUser, UserResponse},
    entity,
};

#[debug_handler]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> AppResult<Json<UserResponse>> {
    let user_model = entity::Entity::find_by_id(id)
        .one(state.db())
        .await?
        .ok_or_else(|| AppError::not_found(format!("user {id} not found")))?;

    let UpdateUser { username, email } = payload;

    if username.is_none() && email.is_none() {
        return Err(AppError::bad_request("at least one field must be provided"));
    }

    let mut active: entity::ActiveModel = user_model.into();

    if let Some(username) = username {
        let trimmed = username.trim();
        if trimmed.is_empty() {
            return Err(AppError::bad_request("username must not be empty"));
        }
        active.username = Set(trimmed.to_owned());
    }

    if let Some(email) = email {
        let trimmed = email.trim();
        if trimmed.is_empty() {
            return Err(AppError::bad_request("email must not be empty"));
        }
        if !trimmed.contains('@') {
            return Err(AppError::bad_request("invalid email format"));
        }
        active.email = Set(trimmed.to_owned());
    }

    active.updated_at = Set(OffsetDateTime::now_utc());

    let updated = active.update(state.db()).await?;

    Ok(Json(UserResponse::from(updated)))
}
