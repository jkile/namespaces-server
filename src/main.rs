use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder };
use mongodb::{Client, options::ClientOptions, Database};
pub mod routes;
pub mod models;


#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

struct AppState {
    pub db: Database
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.app_name = Some("Namespaces".to_string());
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("namespaces");

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                db: db.clone()
            })
            .service(hello)
            .service(routes::user::find_all)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
