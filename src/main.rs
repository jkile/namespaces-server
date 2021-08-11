use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder };
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;

pub mod routes;
pub mod models;
pub mod schema;

// use self::models::*;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub fn create_db_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
    // PgConnection::establish(&database_url)
    //     .expect(&format!("Error connecting to {}", database_url))
}


#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

struct AppState {
    pub connection: PgConnection
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=info");

    let pool = create_db_pool();

    HttpServer::new(move || {
        App::new()
            // .data(AppState {
            //     connection: connection
            // })
            .data(pool.clone())
            .service(hello)
            .service(routes::user::find_all)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
