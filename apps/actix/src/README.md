# RestAPI Impl using actix-web

```rust
impl actix_web::ResponseError for CommonError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(self)
    }
}
```
