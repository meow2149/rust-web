use axum::{Json, extract::State};
use sea_orm::{EntityTrait, QueryOrder};

use crate::{error::AppResult, state::AppState};

use super::{dto::UserResponse, entity};

pub async fn list_users(State(state): State<AppState>) -> AppResult<Json<Vec<UserResponse>>> {
    let users = entity::Entity::find()
        .order_by_asc(entity::Column::Id)
        .all(state.db())
        .await?;

    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}
