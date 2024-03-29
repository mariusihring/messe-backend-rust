use crate::routes::helper::admin::create_salt;
use crate::routes::helper::{generate_token, hasher};
use crate::routes::prisma::{self, admin};
use crate::routes::structs::Login;
use actix_web::web::Json;
use actix_web::HttpResponse;

pub async fn add_admin(login: Json<Login>) -> HttpResponse {
    let token = generate_token().await;
    let client = prisma::new_client().await.unwrap();
    let salt: String = create_salt(10).await;
    let hash = hasher(format!("{}{}", login.password.to_owned(), salt)).await;
    let admin: admin::Data = client
        .admin()
        .create(
            login.username.to_owned(),
            login.email.to_owned(),
            hash,
            token,
            salt,
            vec![],
        )
        .exec()
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("{:?}", admin))
}

pub async fn authenticate_admin(login: Json<Login>) -> HttpResponse {
    let client = prisma::new_client().await.unwrap();
    let user: admin::Data = client
        .admin()
        .find_first(vec![admin::username::equals(login.username.to_owned())])
        .exec()
        .await
        .unwrap()
        .unwrap();

    let hash = hasher(format!(
        "{}{}",
        login.password.to_owned(),
        user.salt.to_owned()
    ))
    .await;
    let json = serde_json::json!({
        "email": &user.email,
        "token": &user.auth_token
    });
    if hash == user.password {
        HttpResponse::Accepted().body(format!("{}", json))
    } else {
        HttpResponse::Unauthorized().body("Not Authorized")
    }
}
