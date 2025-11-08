use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use serde::{Deserialize, Serialize};

use crate::{
    entity::user,
    error::{AppError, AppResult},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}

async fn list_users(State(state): State<AppState>) -> AppResult<Json<Vec<UserResponse>>> {
    let users = user::Entity::find()
        .order_by_asc(user::Column::Id)
        .all(state.db())
        .await?;

    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<UserResponse>> {
    let user = user::Entity::find_by_id(id)
        .one(state.db())
        .await?
        .ok_or_else(|| AppError::not_found(format!("user {id} not found")))?;

    Ok(Json(UserResponse::from(user)))
}

async fn create_user(
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

    let active_model = user::ActiveModel {
        username: Set(username.to_owned()),
        email: Set(email.to_owned()),
        ..Default::default()
    };

    let inserted = active_model.insert(state.db()).await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(inserted))))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> AppResult<Json<UserResponse>> {
    let user_model = user::Entity::find_by_id(id)
        .one(state.db())
        .await?
        .ok_or_else(|| AppError::not_found(format!("user {id} not found")))?;

    let UpdateUser { username, email } = payload;

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
        active.email = Set(trimmed.to_owned());
    }

    active.updated_at = Set(Utc::now());

    let updated = active.update(state.db()).await?;

    Ok(Json(UserResponse::from(updated)))
}

async fn delete_user(State(state): State<AppState>, Path(id): Path<i32>) -> AppResult<StatusCode> {
    let res = user::Entity::delete_by_id(id).exec(state.db()).await?;

    if res.rows_affected == 0 {
        return Err(AppError::not_found(format!("user {id} not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id: i32,
    username: String,
    email: String,
    created_at: String,
    updated_at: String,
}

impl From<user::Model> for UserResponse {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}
