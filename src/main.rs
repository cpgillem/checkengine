use actix_web::{HttpServer, App, web};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

mod responders;
mod models;
mod schema;
mod builders;
mod db_access;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

struct AppState (DbPool);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let db_pool = builders::create_connection();
    let db_pool_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone())
            .route("/registers", web::get().to(responders::get_registers))
            .route("/registers/{id}", web::get().to(responders::get_register))
            .route("/registers", web::post().to(responders::add_register))
            .route("/registers/{id}", web::delete().to(responders::delete_register))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
