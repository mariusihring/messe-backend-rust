use crate::routes::helper::{generate_token, hasher};
use crate::routes::prisma::{self, admin};
use crate::routes::structs::Login;
use actix_web::web::Json;
use actix_web::HttpResponse;

pub async fn add_admin(login: Json<Login>) -> HttpResponse {
    let hash = hasher(login.password.to_owned()).await;
    let token = generate_token().await;
    let client = prisma::new_client().await.unwrap();
    let admin: admin::Data = client
        .admin()
        .create(
            login.username.to_owned(),
            login.email.to_owned(),
            hash,
            token,
            vec![],
        )
        .exec()
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("{:?}", admin))
}

pub async fn authenticate_admin(login: Json<Login>) -> HttpResponse {
    let hash = hasher(login.password.to_owned()).await;
    let client = prisma::new_client().await.unwrap();
    let user: admin::Data = client
        .admin()
        .find_first(vec![
            admin::username::equals(login.username.to_owned()),
            admin::password::equals(hash.to_owned()),
        ])
        .exec()
        .await
        .unwrap()
        .unwrap();

    let json = serde_json::json!({
        "email": &user.email,
        "token": &user.auth_token
    });
    HttpResponse::Ok().body(format!("{}", json))
}
