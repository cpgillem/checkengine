use actix_web::Error;

use crate::{DbPool, models::{posting_group::{InputPostingGroup, PostingGroup}, posting::Posting}, auth::JwtClaims};

#[derive(Clone)]
pub struct PostingGroupController {
    pub pool: DbPool,
}

impl PostingGroupController {
    // pub fn create(&self, input: &InputPostingGroup, jwt: &JwtClaims) -> Result<PostingGroup, Error> {
        
    // }

    // pub fn get_all(&self, jwt: &JwtClaims) -> Result<Vec<PostingGroup>, Error> {

    // }

    // pub fn get(&self, id: i32, jwt: &JwtClaims) -> Result<PostingGroup, Error> {

    // }

    // pub fn delete(&self, id: i32, jwt: &JwtClaims) -> Result<usize, Error> {

    // }

    // pub fn update(&self, id: i32, input: &InputPostingGroup, jwt: &JwtClaims) -> Result<PostingGroup, Error> {

    // }
}