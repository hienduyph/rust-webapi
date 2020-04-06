use actix_web::{web, App, HttpServer};

mod controllers;
mod data;
mod error;
mod identity;
mod infra;
mod middleware;

use crate::data::Data;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // construct di
    let user_component = rwebapi_container::UserContainer::new();

    let user_security_service = user_component.user_security_service.clone();
    let user_service = user_component.user_service.clone();
    let user_service_data = Data::from(user_service.clone());
    let user_auth_service_data = Data::from(user_component.user_auth_service.clone());

    let addr = "0.0.0.0:8000";
    let server = HttpServer::new(move || {
        let auth_middleware = middleware::auth::Auth {
            user_security_service: user_security_service.clone(),
            user_service: user_service.clone(),
        };
        App::new()
            .app_data(user_service_data.clone())
            .app_data(user_auth_service_data.clone())
            .service(
                web::scope("/users")
                    .wrap(auth_middleware)
                    .route("", web::post().to(controllers::create_user))
                    .route("", web::get().to(controllers::get_user))
                    .route("/{id}", web::get().to(controllers::get_user_by_id))
                    .route("/{id}", web::put().to(controllers::update_user))
                    .route("/{id}", web::delete().to(controllers::delete_user)),
            )
            .service(web::scope("/heatlh").route("", web::get().to(controllers::health)))
            .service(web::scope("/auth").route("/login", web::post().to(controllers::login)))
    })
    .bind(addr)?;
    println!("Listening in {}", addr);
    server.run().await
}
