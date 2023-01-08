use crate::DbPool;
use crate::controllers::register_controller::RegisterController;
use crate::controllers::{GetResource, CreateResource, DeleteResource, UpdateResource, GetAllResource};
use crate::responders::get_controller;
use crate::models::register::{InputRegister, UpdateRegister};
use actix_web::{HttpRequest, error};
use actix_web::{web, Error, HttpResponse, get, post, delete, patch};

#[get("")]
pub async fn get_registers(pool: web::Data<DbPool>, request: actix_web::HttpRequest) -> Result<HttpResponse, Error> {
    let controller = get_controller::<RegisterController>(&pool, &request)?;

    // Find the user's registers.
    let registers = controller.get_all()
        .map_err(|e| error::ErrorNotFound(e))?;
    
    Ok(HttpResponse::Ok().json(registers))
}

#[get("{id}")]
pub async fn get_register(pool: web::Data<DbPool>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let controller = get_controller::<RegisterController>(&pool, &request)?;

    // Retrieve the register.
    let register = controller.get(id.into_inner())
        .map_err(|e| error::ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json(register))
}

#[post("")]
pub async fn add_register(pool: web::Data<DbPool>, input: web::Json<InputRegister>, request: actix_web::HttpRequest) -> Result<HttpResponse, Error> {
    let controller = get_controller::<RegisterController>(&pool, &request)?;

    // Create a new register.
    let inserted_register = controller.create(&input)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(inserted_register))
}

#[delete("{id}")]
pub async fn delete_register(pool: web::Data<DbPool>, id: web::Path<i32>, request: actix_web::HttpRequest) ->  Result<HttpResponse, Error> {
    let controller = get_controller::<RegisterController>(&pool, &request)?;

    // Delete the register.
    controller.delete(id.into_inner())
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body("deleted"))
}

#[patch("{id}")]
pub async fn update_register(
    pool: web::Data<DbPool>, 
    id: web::Path<i32>, 
    input: web::Json<UpdateRegister>,  
    request: actix_web::HttpRequest
) -> Result<HttpResponse, Error> {

    let controller = get_controller::<RegisterController>(&pool, &request)?;
    
    let updated_register = controller
        .update(id.into_inner(), &input.into_inner())
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(updated_register))
}