mod prisma;
mod structs;
use actix_web::{get, post, web::Json, web::Path, HttpResponse, Responder};
use prisma::{company_data, interests, user};
use std::fs;
use structs::{NewUser, Person};

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
pub async fn get_specific_user(user_id: Path<i32>) -> impl Responder {
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

#[post("/api/generateData")]
async fn generate_data() -> impl Responder {
    let client = prisma::new_client().await.unwrap();

    let data = fs::read_to_string("./src/names.json").expect("Unable to read file");

    let person: Vec<Person> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    for user in &person {
        client
            .user()
            .create(
                user.family_name.to_owned(),
                user.name.to_owned(),
                format!("{:?}@{:?}.com", &user, &user.family_name),
                "231323123132131".to_owned(),
                vec![],
            )
            .exec()
            .await
            .unwrap();
    }

    HttpResponse::Ok().body("Users succesfully inserted!")
}

/// deserialize `Info` from request's body
#[post("/api/createUser")]
pub async fn create_new_user(user: Json<NewUser>) -> impl Responder {
    let client = prisma::new_client().await.unwrap();

    let data = client
        .user()
        .create(
            user.lastName.to_owned(),
            user.firstName.to_owned(),
            user.mail.to_owned(),
            user.picture.to_owned(),
            vec![
                user::interests::set(
                    interests::web_development::equals(user.interests.coding.to_owned()),
                    interests::cyber_security::equals(user.interests.coding.to_owned()),
                    interests::mobile_dev::equals(user.interests.coding.to_owned()),
                    interests::design::equals(user.interests.coding.to_owned()),
                    interests::data_science::equals(user.interests.coding.to_owned()),
                    interests::coding::equals(user.interests.coding.to_owned()),
                ),
                company_data::is_associated::equals(user.company.isAssociated.to_owned()),
                company_data::company_email::equals(user.company.companyEmail.to_owned()),
                company_data::company_name::equals(user.company.companyName.to_owned()),
            ],
        )
        .exec()
        .await;

    HttpResponse::Ok()
}
