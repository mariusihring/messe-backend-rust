mod prisma;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use prisma::PrismaClient;
use prisma::{company_data, interests, user};
use prisma_client_rust::NewClientError;
use std::any::type_name;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_all_users)
            .service(get_specific_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from the Messe API")
}
#[get("/api/getAllUsers")]
async fn get_all_users() -> impl Responder {
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
#[get("/api/getSpecificUser/{user_id}")]
async fn get_specific_user(user_id: web::Path<i32>) -> impl Responder {
    let user_id: i32 = user_id.to_owned();
    let client = prisma::new_client().await.unwrap();
    let user = client
        .user()
        .find_first(vec![user::id::equals(user_id)])
        .with(user::interests::fetch())
        .with(user::company_data::fetch())
        .exec()
        .await
        .unwrap();
    let json = serde_json::to_string(&user).unwrap();
    HttpResponse::Ok().body(json)
}
