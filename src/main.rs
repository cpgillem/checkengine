use std::env;

use actix_web::{HttpServer, App, web};
use diesel::{PgConnection, Connection};
use dotenvy::dotenv;

struct AppState {
    database_connection: PgConnection,
}

pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Could not connect to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                database_connection: create_connection(),
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
