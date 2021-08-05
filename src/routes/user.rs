use crate::models::{user::User};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/users")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        User {
            id: String::from("123"),
            username: String::from("jkile9"),
            password: String::from("asdfasdfasdf"),
            first_name: String::from("Jake"),
            last_name: String::from("Kile"),
            // avatar: ImageUrl,
            // comments: vec![]
        }
    ])
}
