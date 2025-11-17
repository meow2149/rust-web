use axum::debug_handler;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use time::OffsetDateTime;

use crate::{
    dto::user::{CreateUser, UpdateUser, UserResponse},
    entity::user,
    error::{AppError, AppResult},
    state::AppState,
};

#[debug_handler]
pub async fn list_users(State(state): State<AppState>) -> AppResult<Json<Vec<UserResponse>>> {
    let users = user::Entity::find()
        .order_by_asc(user::Column::Id)
        .all(state.db())
        .await?;

    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

#[debug_handler]
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

    let active_model = user::ActiveModel {
        username: Set(username.to_owned()),
        email: Set(email.to_owned()),
        ..Default::default()
    };

    let inserted = active_model.insert(state.db()).await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(inserted))))
}

#[debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<UserResponse>> {
    let user = user::Entity::find_by_id(id)
        .one(state.db())
        .await?
        .ok_or_else(|| AppError::not_found(format!("user {id} not found")))?;

    Ok(Json(UserResponse::from(user)))
}

#[debug_handler]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> AppResult<Json<UserResponse>> {
    let user_model = user::Entity::find_by_id(id)
        .one(state.db())
        .await?
        .ok_or_else(|| AppError::not_found(format!("user {id} not found")))?;

    let UpdateUser { username, email } = payload;

    if username.is_none() && email.is_none() {
        return Err(AppError::bad_request("at least one field must be provided"));
    }

    let mut active: user::ActiveModel = user_model.into();

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

#[debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<StatusCode> {
    let res = user::Entity::delete_by_id(id).exec(state.db()).await?;

    if res.rows_affected == 0 {
        return Err(AppError::not_found(format!("user {id} not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}
