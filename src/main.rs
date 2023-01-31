mod prisma;
use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use prisma::PrismaClient;
use prisma::{company_data, interests, user};
use prisma_client_rust::NewClientError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(getAllUsers))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

// All The Routes
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
#[get("/api/getAllUsers")]
async fn getAllUsers() -> impl Responder {
    let client = prisma::new_client().await.unwrap();
    let users: Vec<user::Data> = client
        .user()
        .find_many(vec![])
        .with(user::interests::fetch())
        .with(user::company_data::fetch())
        .exec()
        .await
        .unwrap();
    let json = serde_json::to_string(&users).unwrap();
    HttpResponse::Ok().body(json)
}
