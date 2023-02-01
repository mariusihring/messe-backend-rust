mod prisma;
use actix_web::{get, post, web, web::Json, App, HttpResponse, HttpServer, Responder};
use prisma::user;
mod structs;
use structs::NewUser;

#[get("/api/getAllUsers")]
pub async fn get_all_users() -> impl Responder {
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
pub async fn get_specific_user(user_id: web::Path<i32>) -> impl Responder {
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

#[post("/api/createUser")]
pub async fn create_new_user(body: Json<NewUser>) -> impl Responder {}
