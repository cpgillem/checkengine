use actix_web::{HttpServer, App, web};
use checkengine::{*};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let db_pool = builders::create_connection();
    let db_pool_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/register")
                            .service(responders::register::get_registers)
                            .service(responders::register::add_register)
                            .service(responders::register::get_register)
                            .service(responders::register::delete_register)
                            .service(responders::register::update_register)
                    )
                    .service(
                        web::scope("/posting_group")
                            .service(responders::posting_group::create_posting_group)
                            .service(responders::posting_group::get_posting_group)
                            .service(responders::posting_group::delete_posting_group)
                            .service(responders::posting_group::update_posting_group)
                            .service(responders::posting_group::get_posting_groups)
                    )
                    .service(
                        web::scope("/posting")
                            .service(responders::posting::get_posting)
                            .service(responders::posting::add_posting)
                            .service(responders::posting::update_posting)
                            .service(responders::posting::delete_posting)
                    )
                    .service(
                        web::scope("/auth")
                            .service(responders::auth::authenticate)
                            .service(responders::auth::signup)
                    )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
