use crate::entity::error;
use crate::entity::user;

pub trait UserService: Send + Sync {
    fn users(&self) -> Result<Vec<user::User>, error::CommonError>;
}

pub struct UserServiceImpl {}

impl UserService for UserServiceImpl {
    fn users(&self) -> Result<Vec<user::User>, error::CommonError> {
        let users = vec![user::User {
            id: "111".into(),
            first_name: "Mock".into(),
            last_name: "Hehe".into(),
            email: "hh@tiki.vn".into(),
            password: "11".into(),
            created_by: "1".into(),
            created_at: chrono::Local::now().naive_local(),
            updated_by: "2".into(),
            updated_at: chrono::Local::now().naive_local(),
        }];
        Ok(users)
    }
}
