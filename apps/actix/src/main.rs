use actix_web::{web, App, HttpServer};
use rwebapi_users;

mod controllers;
mod error;
mod infra;

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(controllers::health))
        .service(
            web::scope("/users")
                .route("", web::post().to(controllers::create_user))
                .route("", web::get().to(controllers::get_user)),
        );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // construct di
    let svc: Box<&dyn rwebapi_users::UserService> = Box::new(&rwebapi_users::UserServiceImpl {});
    let user_services = web::Data::new(svc);

    let addr = "0.0.0.0:8000";
    let server =
        HttpServer::new(move || App::new().app_data(user_services.clone()).configure(routes))
            .bind(addr)?;
    println!("Listenign in {}", addr);
    server.run().await
}
