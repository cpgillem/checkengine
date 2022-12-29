mod responders;
mod models;
mod schema;
mod builders;
mod db_access;
mod auth;

use actix_web::{HttpServer, App, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let db_pool = builders::create_connection();
    let db_pool_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone())
            .service(
                web::scope("/api/v1").service(
                    web::scope("/register")
                        .service(responders::register::get_registers)
                        .service(responders::register::add_register)
                        .service(responders::register::get_register)
                        .service(responders::register::delete_register)
                        .service(responders::register::update_register)
                )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
