use actix_web::{HttpServer, App, web};
use checkengine::{*, controllers::member_controller::MemberController};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let db_pool = builders::create_connection();
    let db_pool_data = web::Data::new(db_pool);

    let member_controller = MemberController {
        pool: builders::create_connection(),
    };
    let member_controller_data = web::Data::new(member_controller);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone())
            .app_data(member_controller_data.clone())
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
