use actix_web::{web, App, HttpServer};

pub async fn serve() -> std::io::Result<()> {
    // construct di
    let user_component = crate::container::UserContainer::new();

    let user_security_service = user_component.user_security_service.clone();
    let user_service = user_component.user_service.clone();
    let user_service_data = web::Data::from(user_service.clone());
    let user_auth_service_data = web::Data::from(user_component.user_auth_service.clone());

    let addr = "0.0.0.0:8000";
    let server = HttpServer::new(move || {
        let auth_middleware = super::auth_middleware::Auth {
            user_security_service: user_security_service.clone(),
            user_service: user_service.clone(),
        };
        App::new()
            .app_data(user_service_data.clone())
            .app_data(user_auth_service_data.clone())
            .service(
                web::scope("/users")
                    .wrap(auth_middleware)
                    .route("", web::post().to(super::user_handler::create_user))
                    .route("", web::get().to(super::user_handler::get_user))
                    .route("/{id}", web::get().to(super::user_handler::get_user_by_id))
                    .route("/{id}", web::put().to(super::user_handler::update_user))
                    .route("/{id}", web::delete().to(super::user_handler::delete_user)),
            )
            .service(web::scope("/health").route("", web::get().to(super::health_handler::health)))
            .service(
                web::scope("/auth").route("/login", web::post().to(super::auth_handler::login)),
            )
            .service(web::scope("/").route("", web::get().to(super::health_handler::health)))
    })
    .bind(addr)?;
    println!("Listening in {}", addr);
    server.run().await
}
