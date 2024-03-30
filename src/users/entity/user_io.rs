use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::User;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct UserIdentity {
    pub email: String,
    pub user_id: String,
}

impl CreateUserRequest {
    pub fn to_user(&self, created_by: String) -> User {
        let now = Utc::now().naive_utc();
        let user_id = Uuid::new_v4().to_string();
        User {
            id: user_id.to_string(),
            email: self.email.to_string(),
            first_name: self.first_name.to_string(),
            last_name: self.last_name.to_string(),
            updated_at: now,
            updated_by: created_by.clone(),
            created_at: now,
            created_by: created_by.clone(),
            password: self.password.to_string(),
        }
    }
}
