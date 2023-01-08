use crate::controllers::{CreateResource, DeleteResource, UpdateResource, GetResource};
use crate::controllers::posting_controller::PostingController;
use crate::models::posting::UpdatePosting;
use crate::models::posting::InputPosting;
use crate::responders::get_controller;

use crate::DbPool;
use actix_web::{web, HttpResponse, HttpRequest, Error, post, delete, error, patch, get};

/// Adds a posting to the posting group.
#[post("")]
pub async fn add_posting(pool: web::Data<DbPool>, request: HttpRequest, input_posting: web::Json<InputPosting>) -> Result<HttpResponse, Error> {
    let controller = get_controller::<PostingController>(&pool, &request)?;
    let new_posting = controller.create(&input_posting)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(new_posting))
}

/// Deletes a posting from a posting group. 
#[delete("{id}")]
pub async fn delete_posting(pool: web::Data<DbPool>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let controller = get_controller::<PostingController>(&pool, &request)?;
    controller.delete(id.into_inner()).map_err(|e| error::ErrorNotFound(e))?;
    Ok(HttpResponse::Ok().finish())
}

/// Updates a posting.
#[patch("{id}")]
pub async fn update_posting(pool: web::Data<DbPool>, id: web::Path<i32>, update_posting: web::Json<UpdatePosting>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let controller = get_controller::<PostingController>(&pool, &request)?;
    let updated_posting = controller.update(id.into_inner(), &update_posting)
        .map_err(|e| error::ErrorNotFound(e))?;
    Ok(HttpResponse::Ok().json(updated_posting))
}

// Gets one posting.
#[get("{id}")]
pub async fn get_posting(pool: web::Data<DbPool>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let controller = get_controller::<PostingController>(&pool, &request)?;
    let posting = controller.get(id.into_inner())
        .map_err(|e| error::ErrorNotFound(e))?;
    Ok(HttpResponse::Ok().json(posting))
}