# Users component

Collects utilties related to users
- CRUD
- Authentication & Authorization

## Rules
- Only public entity & services
- No web framework presents here, no actix, no warp
- Services always use repo interface, not the actual implementation
- Do not public you repo implementation

```rust
impl From<NewUser> for User {
  fn from(user: NewUser) -> Self {
      let now = Utc::now().naive_utc();
      User {
          id: user.id,
          first_name: user.first_name,
          last_name: user.last_name,
          email: user.email,
          // TODO: hash funtion
          password: "".to_owned(),
          created_by: user.created_by,
          created_at: now,
          updated_by: user.updated_by,
          updated_at: now,
      }
  }
}
``````
