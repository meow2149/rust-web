use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;

use super::entity;

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<entity::Model> for UserResponse {
    fn from(model: entity::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            created_at: model.created_at.format(&Rfc3339).unwrap_or_default(),
            updated_at: model.updated_at.format(&Rfc3339).unwrap_or_default(),
        }
    }
}
