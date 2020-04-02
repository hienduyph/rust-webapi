#[macro_use]
extern crate diesel;
use actix_web::{web, App, HttpServer};

mod controllers;
mod entity;
mod infra;
mod repo;
mod services;

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(controllers::health::handle))
        .service(
            web::scope("/users")
                .route("", web::post().to(controllers::user::create_user))
                .route("", web::get().to(controllers::user::get_user)),
        );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // construct di
    let svc: Box<&dyn services::users::UserService> =
        Box::new(&services::users::UserServiceImpl {});
    let user_services = web::Data::new(svc);
    let addr = "0.0.0.0:8000";
    let server =
        HttpServer::new(move || App::new().app_data(user_services.clone()).configure(routes))
            .bind(addr)?;
    println!("Listenign in {}", addr);
    server.run().await
}
