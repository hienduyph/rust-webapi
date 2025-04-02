# Backend Implementation in Rust with Clean Architect

- Clean Architect
- Independent framework: I've tried actix-web, warp, ...
- Independent database: Support MySQL, PostgresSQL ...

## Run the services
```bash
docker compose up -d
```

Setup Migrations

```bash
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/mydb

cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"
diesel setup
diesel migration run

# Insert sample users
cargo run --bin insert_users
```

**Check the server's running**

```bash
curl http://127.0.0.1:8000/
curl http://127.0.0.1:8000/health
```

**Login**

```bash
# required jq installed.
export TOKEN=$(curl -s -H "Content-Type: application/json" -X POST -d '{"email": "hienduyph@gmail.com", "password": "admin"}' http://127.0.0.1:8000/auth/login | jq -r '.token')
```

**Users**

```bash
curl -s -H "Authorization: bearer $TOKEN" http://127.0.0.1:8000/users
curl -s -H "Authorization: bearer $TOKEN" http://127.0.0.1:8000/users/1802d2f8-1a18-43c1-9c58-1c3f7100c842
```

## Dev Started
**Run the server**

- [axum](./src/apps/axum): `cargo run --bin axum`
- [warp](./src/apps/warp): `cargo run --bin warp`
- [actix-web](./src/apps/actix): `cargo run --bin actix`

