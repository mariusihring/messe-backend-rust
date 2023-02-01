mod prisma;
use std::{fs::File, io::BufReader};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use prisma::user;
use serde::Deserialize;
use std::fs;
use std::io::{self, prelude::*};
use std::io::prelude::*;

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

#[derive(Deserialize)]
struct Info {
    ammount_user: i32,
}

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    family_name: String
}

#[post("/api/generateData")]
async fn generate_data(info: web::Json<Info>) -> impl Responder {
    let client = prisma::new_client().await.unwrap();

    let data = fs::read_to_string("./src/names.json")
    .expect("Unable to read file");

    let person: Vec<Person> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    for user in &person {
        let new_user = client
        .user()
        .create(
            user.family_name.to_owned(),
            user.name.to_owned(),
            format!("{:?}@{:?}.com", &user, &user.family_name),
            "231323123132131".to_owned(),
            vec![]
        )
        .exec()
        .await;

        match new_user {
            Ok(_) => HttpResponse::Ok(),
            Err(_) => HttpResponse::Ok()
        };
        
    }
    


    HttpResponse::Ok().body(format!("Welcome {}!", info.ammount_user))
}



/// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.ammount_user))
}

