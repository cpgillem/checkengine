use actix_web::{HttpResponse, error::Error, get};

#[get("")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("success"))
}