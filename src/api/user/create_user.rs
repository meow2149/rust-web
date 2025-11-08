use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, Set};

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

use super::{
    dto::{CreateUser, UserResponse},
    entity,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> AppResult<(StatusCode, Json<UserResponse>)> {
    let CreateUser { username, email } = payload;

    let username = username.trim();
    if username.is_empty() {
        return Err(AppError::bad_request("username must not be empty"));
    }

    let email = email.trim();
    if email.is_empty() {
        return Err(AppError::bad_request("email must not be empty"));
    }
    if !email.contains('@') {
        return Err(AppError::bad_request("invalid email format"));
    }

    let active_model = entity::ActiveModel {
        username: Set(username.to_owned()),
        email: Set(email.to_owned()),
        ..Default::default()
    };

    let inserted = active_model.insert(state.db()).await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(inserted))))
}
