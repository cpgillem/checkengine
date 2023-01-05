use crate::models::posting::{Posting, NewPosting};
use crate::models::{member::Member, posting::InputPosting};
use crate::models::posting_group::*;
use crate::responders::{get_connection, get_member};
use crate::{schema::*, DbConnection};

use crate::DbPool;
use actix_web::{web, HttpResponse, HttpRequest, Error, get, post, delete, patch, error};
use diesel::dsl::now;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use chrono::prelude::*;

/// Gets a posting group by ID, along with its postings.
#[get("{id}")]
pub async fn get_posting_group(pool: web::Data<DbPool>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let member = get_member(&request, &pool)?;
    let mut connection = get_connection(&pool)?;

    let posting_group = posting_group::table
        .filter(posting_group::member_id.eq(member.id))
        .filter(posting_group::id.eq(id))
        .first::<PostingGroup>(&mut connection)
        .map_err(|e| error::ErrorNotFound(e))?;

    let postings = Posting::belonging_to(&posting_group)
        .load::<Posting>(&mut connection)
        .map_err(|e| error::ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json(FullPostingGroup::new(&posting_group, &postings)))
}

/// Creates a posting group given the group metadata and all postings. They must balance.
#[post("")]
pub async fn create_posting_group(pool: web::Data<DbPool>, request: HttpRequest, input_posting_group: web::Json<InputPostingGroup>) -> Result<HttpResponse, Error> {
    let member = get_member(&request, &pool)?;
    let mut connection = get_connection(&pool)?;
    let new_posting_group = NewPostingGroup::from_input(&input_posting_group.0, &member);

    // Add the group to the database.
    let inserted_posting_group = diesel::insert_into(posting_group::table)
        .values(&new_posting_group)
        .get_result::<PostingGroup>(&mut connection)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(inserted_posting_group))
}

/// Deletes a posting group.
#[delete("{id}")]
pub async fn delete_posting_group(pool: web::Data<DbPool>, path: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let member = get_member(&request, &pool)?;
    let mut connection = get_connection(&pool)?;

    // Delete the posting group.
    diesel::delete(
        posting_group::table
            .filter(posting_group::member_id.eq(member.id))
            .find(id)
    )
    .execute(&mut connection)
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body("deleted"))
}

/// Updates metadata for a posting group.
#[patch("{id}")]
pub async fn update_posting_group(pool: web::Data<DbPool>, path: web::Path<i32>, request: HttpRequest, input_posting_group: web::Json<InputPostingGroup>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let member = get_member(&request, &pool)?;
    let mut connection = get_connection(&pool)?;

    // Update the posting group.
    diesel::update(
        posting_group::table
            .filter(posting_group::member_id.eq(member.id))
            .find(id)
    )
    .set((
        posting_group::posted_at.eq(input_posting_group.posted_at),
        posting_group::check_number.eq(input_posting_group.check_number.clone()),
        posting_group::summary.eq(input_posting_group.summary.clone()),
        posting_group::modified_at.eq(now),
    ))
    .execute(&mut connection)
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().finish())
}