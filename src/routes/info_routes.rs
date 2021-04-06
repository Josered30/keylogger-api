use crate::errors::ApiError;
use crate::models::Info;
use actix_web::{get, post, web, HttpResponse};

#[post("/api/logs")]
async fn post_logs(info: web::Json<Info>) -> Result<HttpResponse, ApiError> {
    let result = Info::save(info.into_inner())?;
    return Ok(HttpResponse::Ok().json(result));
}

#[get("/api/logs")]
async fn get_logs(info: web::Json<Info>) -> Result<HttpResponse, ApiError> {
    let info = Info::get(info.into_inner())?;
    return Ok(HttpResponse::Ok().json(info));
}

pub fn init_info_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_logs);
    cfg.service(get_logs);
}
