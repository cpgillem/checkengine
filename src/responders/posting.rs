use crate::models::posting::{Posting, NewPosting};
use crate::models::posting::InputPosting;
use crate::models::posting_group::*;
use crate::responders::{get_connection, get_member};
use crate::schema::*;

use crate::DbPool;
use actix_web::{web, HttpResponse, HttpRequest, Error, post, delete, error};
use diesel::prelude::*;
use diesel::RunQueryDsl;

/// Adds a posting to the posting group.
#[post("posting")]
pub async fn add_posting(pool: web::Data<DbPool>, request: HttpRequest, input_posting: web::Json<InputPosting>) -> Result<HttpResponse, Error> {
    let member = get_member(&request, &pool)?;
    let mut connection = get_connection(&pool)?;

    // Retrieve the group to validate its ownership.
    let posting_group = posting_group::table
        .filter(posting_group::member_id.eq(member.id))
        .filter(posting_group::id.eq(input_posting.0.posting_group_id))
        .first::<PostingGroup>(&mut connection)
        .map_err(|e| error::ErrorNotFound(e))?;

    // Validate ownership.
    if posting_group.member_id != member.id {
        return Err(error::ErrorUnauthorized("not owned"));
    }

    // Insert the new posting.
    let new_posting = NewPosting::from_input(&input_posting.0);
    let inserted_posting = diesel::insert_into(posting::table)
        .values(&new_posting)
        .get_result::<Posting>(&mut connection)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(inserted_posting))
}

/// Deletes a posting from a posting group. 
#[delete("posting/{id}")]
pub async fn delete_posting(pool: web::Data<DbPool>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let member = get_member(&request, &pool)?;
    let mut connection = get_connection(&pool)?;

    // Retrieve the posting.
    let posting = posting::table
        .filter(posting::id.eq(id.into_inner()))
        .first::<Posting>(&mut connection)
        .map_err(|e| error::ErrorNotFound(e))?;

    // Retrieve the parent group for validation.
    let posting_group = posting_group::table
        .filter(posting_group::id.eq(posting.posting_group_id))
        .first::<PostingGroup>(&mut connection)
        .map_err(|e| error::ErrorNotFound(e))?;

    // Validate ownership.    
    if posting_group.member_id != member.id {
        return Err(error::ErrorUnauthorized("not owned"));
    }

    // Delete the posting.
    diesel::delete(
            posting::table
                .filter(posting::id.eq(posting.id))
        )
        .execute(&mut connection)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().body("deleted"))
}