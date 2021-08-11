// use crate::models::{user::User};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/users")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok()
}
