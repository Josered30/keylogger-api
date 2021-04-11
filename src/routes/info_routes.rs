use std::error::Error;

use crate::models::Info;
use crate::{errors::ApiError, main};
use actix_web::{delete, get, post, web, HttpResponse};

#[post("/api/logs")]
async fn post_logs(info: web::Json<Info>) -> Result<HttpResponse, ApiError> {
    let result = Info::save(info.into_inner())?;
    return Ok(HttpResponse::Ok().json(result));
}

#[get("/api/logs")]
async fn get_logs(web::Query(info): web::Query<Info>) -> Result<HttpResponse, ApiError> {
    let info = web::block(move || Info::get(info.filename)).await?;
    return Ok(HttpResponse::Ok().json(info));
}

#[get("/api/filenames")]
async fn get_filenames() -> Result<HttpResponse, ApiError> {
    let info = Info::get_filenames()?;
    return Ok(HttpResponse::Ok().json(info));
}

#[delete("/api/logs")]
async fn delete_log(web::Query(info): web::Query<Info>) -> Result<HttpResponse, ApiError> {
    let result = Info::delete_log(info.filename)?;
    return Ok(HttpResponse::Ok().json(result));
}

pub fn init_info_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_logs);
    cfg.service(get_logs);
    cfg.service(get_filenames);
    cfg.service(delete_log);
}
