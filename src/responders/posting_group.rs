use crate::DbPool;
use crate::controllers::posting_group_controller::PostingGroupController;
use crate::controllers::{GetResource, CreateResource, DeleteResource, UpdateResource, GetChildren, GetAllResource};
use crate::models::posting_group::*;
use crate::responders::get_controller;
use actix_web::{web, HttpResponse, HttpRequest, Error, get, post, delete, patch, error};

/// Gets a posting group by ID, along with its postings.
#[get("{id}")]
pub async fn get_posting_group(pool: web::Data<DbPool>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let controller = get_controller::<PostingGroupController>(&pool, &request)?;

    let posting_group = controller.get(id.into_inner())
        .map_err(|e| error::ErrorNotFound(e))?;

    let postings = controller.get_children(&posting_group)
        .map_err(|e| error::ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json((&posting_group, &postings)))
}

// Gets all posting groups from a user. Probably the most important responder.
#[get("")]
pub async fn get_posting_groups(pool: web::Data<DbPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let controller = get_controller::<PostingGroupController>(&pool, &request)?;

    let posting_groups = controller.get_all()
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(posting_groups))
}

/// Creates a posting group given the group metadata and all postings. They must balance.
#[post("")]
pub async fn create_posting_group(pool: web::Data<DbPool>, request: HttpRequest, input_posting_group: web::Json<InputPostingGroup>) -> Result<HttpResponse, Error> {
    let controller = get_controller::<PostingGroupController>(&pool, &request)?;
    let inserted_posting_group = controller.create(&input_posting_group)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(inserted_posting_group))
}

/// Deletes a posting group.
#[delete("{id}")]
pub async fn delete_posting_group(pool: web::Data<DbPool>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let controller = get_controller::<PostingGroupController>(&pool, &request)?;
    
    // Delete the posting group.
    controller.delete(id.into_inner())
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body("deleted"))
}

/// Updates metadata for a posting group.
#[patch("{id}")]
pub async fn update_posting_group(pool: web::Data<DbPool>, path: web::Path<i32>, request: HttpRequest, input_posting_group: web::Json<UpdatePostingGroup>) -> Result<HttpResponse, Error> {
    let controller = get_controller::<PostingGroupController>(&pool, &request)?;
    
    // Update the posting group.
    let updated_posting_group = controller.update(path.into_inner(), &input_posting_group)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(updated_posting_group))
}
