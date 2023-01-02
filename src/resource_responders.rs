use std::marker::PhantomData;

use diesel::prelude::*;
use diesel::query_builder::AsQuery;
use diesel::query_dsl::LoadQuery;
use diesel::{RunQueryDsl};
use serde::Serialize;
use crate::{DbPool, DbConnection};
use actix_web::{web, Error, HttpResponse, error};

/// The model is the type of your model including the ID.
/// The table is the table object from your schema.
pub struct Controller<'a, TModel: Serialize, TTable: RunQueryDsl<DbConnection> + AsQuery + LoadQuery<'a, DbConnection, TModel> + Copy> {
    pub table: &'a TTable,
    tmodel: PhantomData<TModel>,
}

pub fn create_controller<'a, TModel: Serialize, TTable: RunQueryDsl<DbConnection> + AsQuery + LoadQuery<'a, DbConnection, TModel> + Copy>(t: &'a TTable) -> Controller<'a, TModel, TTable> {
    Controller::<TModel, TTable> {
        table: t,
        tmodel: PhantomData,
    }
}

impl<'a, TModel: Serialize, TTable: RunQueryDsl<DbConnection> + AsQuery + LoadQuery<'a, DbConnection, TModel> + Copy> Controller<'a, TModel, TTable> {
    pub async fn get_all(&self, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
        let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

        match self.table.load::<TModel>(&mut connection) {
            Ok(v) => Ok(HttpResponse::Ok().json(v)),
            _ => Ok(HttpResponse::InternalServerError().body("fail")),
        }
    }
}

