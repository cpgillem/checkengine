use actix_web::{HttpServer, App, web};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

mod responders;
mod models;
mod schema;
mod builders;
mod db_access;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let db_pool = builders::create_connection();
    let db_pool_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone())
            .service(responders::get_registers)
            .service(responders::add_register)
            .route("/registers/{id}", web::get().to(responders::get_register))
            .route("/registers/{id}", web::delete().to(responders::delete_register))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
