use crate::controllers::posting_group_controller::PostingGroupController;
use crate::models::posting::Posting;
use crate::models::posting_group::*;
use crate::responders::{get_connection, get_jwt};
use actix_web::{web, HttpResponse, HttpRequest, Error, get, post, delete, patch, error};
use diesel::prelude::*;
use diesel::RunQueryDsl;

/// Gets a posting group by ID, along with its postings.
#[get("{id}")]
pub async fn get_posting_group(controller: web::Data<PostingGroupController>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let jwt = get_jwt(&request)?;
    let mut connection = get_connection(&controller.pool)?;

    let posting_group = controller.get(id.into_inner(), &jwt)
        .map_err(|e| error::ErrorNotFound(e))?;

    let postings = Posting::belonging_to(&posting_group)
        .load::<Posting>(&mut connection)
        .map_err(|e| error::ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json((&posting_group, &postings)))
}

/// Creates a posting group given the group metadata and all postings. They must balance.
#[post("")]
pub async fn create_posting_group(controller: web::Data<PostingGroupController>, request: HttpRequest, input_posting_group: web::Json<InputPostingGroup>) -> Result<HttpResponse, Error> {
    let jwt = get_jwt(&request)?;
    let inserted_posting_group = controller.create(&input_posting_group, &jwt)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(inserted_posting_group))
}

/// Deletes a posting group.
#[delete("{id}")]
pub async fn delete_posting_group(controller: web::Data<PostingGroupController>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let jwt = get_jwt(&request)?;
    
    // Delete the posting group.
    controller.delete(id.into_inner(), &jwt)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body("deleted"))
}

/// Updates metadata for a posting group.
#[patch("{id}")]
pub async fn update_posting_group(controller: web::Data<PostingGroupController>, path: web::Path<i32>, request: HttpRequest, input_posting_group: web::Json<UpdatePostingGroup>) -> Result<HttpResponse, Error> {
    let jwt = get_jwt(&request)?;
    // Update the posting group.
    let updated_posting_group = controller.update(path.into_inner(), &input_posting_group, &jwt)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(updated_posting_group))
}
