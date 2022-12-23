use std::env;

use actix_web::{HttpServer, App, web, Responder, HttpResponse, get};
use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

struct AppState {
    db_pool: Pool,
}

pub fn create_connection() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create DB connection pool.")
    // PgConnection::establish(&database_url)
    //     .unwrap_or_else(|_| panic!("Could not connect to {}", database_url))
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: create_connection(),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
