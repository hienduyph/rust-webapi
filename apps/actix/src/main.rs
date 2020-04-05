use actix_web::{web, App, HttpServer};
use rwebapi_users;

mod controllers;
mod error;
mod infra;

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(controllers::create_user))
            .route("", web::get().to(controllers::get_user))
            .route("/{id}", web::get().to(controllers::get_user_by_id))
            .route("/{id}", web::put().to(controllers::update_user))
            .route("/{id}", web::delete().to(controllers::delete_user)),
    )
    .service(web::scope("/heatlh").route("", web::get().to(controllers::health)))
    .service(web::scope("/auth").route("/login", web::post().to(controllers::login)));
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // construct di
    let user_component = rwebapi_container::UserContainer::new();
    let user_services =
        web::Data::new(user_component.user_service as Box<dyn rwebapi_users::UserService>);
    let user_auth_service =
        web::Data::new(user_component.user_auth_service as Box<dyn rwebapi_users::UserAuthService>);

    let addr = "0.0.0.0:8000";
    let server = HttpServer::new(move || {
        App::new()
            .app_data(user_services.clone())
            .app_data(user_auth_service.clone())
            .configure(routes)
    })
    .bind(addr)?;
    println!("Listening in {}", addr);
    server.run().await
}
