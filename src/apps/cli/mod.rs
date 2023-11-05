use chrono::Utc;

use crate::users::User;

pub async fn run() -> std::io::Result<()> {
    let cmp = crate::container::UserContainer::new().await;
    let user_service = cmp.user_service;
    let now = Utc::now().naive_utc();
    let user = User {
        id: "1802d2f8-1a18-43c1-9c58-1c3f7100c842".into(),
        first_name: "Hien".into(),
        last_name: "Pham".into(),
        email: "hienduyph@gmail.com".into(),
        updated_at: now,
        updated_by: "admin".into(),
        created_at: now,
        created_by: "00000000-0000-0000-0000-000000000000".into(),
        password: "admin".into(),
    };
    user_service.create(&user).await.unwrap();
    Ok(())
}
