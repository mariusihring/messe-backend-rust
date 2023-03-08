pub mod prisma;
mod structs;

use actix_web::{get, post, put, web::Json, web::Path, HttpResponse, Responder};

use prisma::{company_data, interests, user};
use prisma_client_rust::query_core::interpreter;
use std::fs;
use structs::{DbInterests, DbUser, Interests, NewUser, Person};

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

pub async fn get_specific_user(user_mail: Path<String>) -> impl Responder {
    let client = prisma::new_client().await.unwrap();
    let user = client
        .user()
        .find_first(vec![user::mail::equals(user_mail.to_owned())])
        .with(user::interests::fetch())
        .with(user::company_data::fetch())
        .exec()
        .await;

    match user {
        Ok(data) => {
            let json = serde_json::to_string(&data).unwrap();
            HttpResponse::Ok().body(json)
        }
        Err(err) => HttpResponse::BadRequest().body(format!(
            "Could not get user. Following Error occured: {}",
            err
        )),
    }
}

pub async fn generate_data() -> impl Responder {
    let client = prisma::new_client().await.unwrap();

    let data = fs::read_to_string("./src/names.json").expect("Unable to read file");

    let person: Vec<Person> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    for user in &person {
        client
            .user()
            .create(
                user.family_name.to_owned(),
                user.name.to_owned(),

                format!("{}@{}.com", &user.name, &user.family_name),

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

pub async fn create_new_user(user: Json<NewUser>) -> HttpResponse {
    let client = prisma::new_client().await.unwrap();
    let created_user = client
        .user()
        .create(
            user.lastName.to_owned(),
            user.firstName.to_owned(),
            user.mail.to_owned(),
            user.picture.to_owned(),
            vec![],
        )
        .exec()
        .await
        .unwrap();
    let data: (Vec<company_data::Data>, Vec<interests::Data>) = client
        ._batch((
            vec![client.company_data().create(
                user.company.isAssociated,
                user.company.companyEmail.to_owned(),
                user.company.companyName.to_owned(),
                user::id::equals(created_user.id),
                vec![],
            )],
            vec![client.interests().create(
                user.interests.webDevelopment,
                user.interests.cyberSecurity,
                user.interests.mobileDev,
                user.interests.design,
                user.interests.dataScience,
                user.interests.coding,
                user::id::equals(created_user.id),
                vec![],
            )],
        ))
        .await
        .unwrap();

    HttpResponse::Ok().body(format!{"User for mail {} successfully created with id {}", created_user.mail, created_user.id})
}


#[delete("/api/deleteUser/{user_id}")]

pub async fn delete_user(user_id: Path<i32>) -> impl Responder {
    let client = prisma::new_client().await.unwrap();
    let _data: (
        Vec<interests::Data>,
        Vec<company_data::Data>,
        Vec<user::Data>,
    ) = client
        ._batch((
            vec![client
                .interests()
                .delete(interests::user_id::equals(user_id.to_owned()))],
            vec![client
                .company_data()
                .delete(company_data::user_id::equals(user_id.to_owned()))],
            vec![client.user().delete(user::id::equals(user_id.to_owned()))],
        ))
        .await
        .unwrap();
    HttpResponse::Ok().body(format!("user with id {} successfully deleted", user_id))
}


pub async fn number_of_users() -> impl Responder {
    let client = prisma::new_client().await.unwrap();
    let data = client.user().count(vec![]).exec().await;
    match data {
        Ok(num) => HttpResponse::Ok().body(format!("{{ \"numOfUsers\": {} }}", num)),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

pub async fn number_of_associates() -> impl Responder {
    let client = prisma::new_client().await.unwrap();
    let data = client.company_data().count(vec![]).exec().await;
    match data {
        Ok(num) => HttpResponse::Ok().body(format!("{{ \"numOfAssociates\": {} }}", num)),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

pub async fn num_of_interest() -> impl Responder {
    let client = prisma::new_client().await.unwrap();
    let (num_web_dev, num_cyber_sec, num_mobile_dev, num_design, num_data_science, num_coding): (
        Vec<i64>,
        Vec<i64>,
        Vec<i64>,
        Vec<i64>,
        Vec<i64>,
        Vec<i64>,
    ) = client
        ._batch((
            vec![client
                .interests()
                .count(vec![interests::web_development::equals(true)])],
            vec![client
                .interests()
                .count(vec![interests::cyber_security::equals(true)])],
            vec![client
                .interests()
                .count(vec![interests::mobile_dev::equals(true)])],
            vec![client
                .interests()
                .count(vec![interests::design::equals(true)])],
            vec![client
                .interests()
                .count(vec![interests::data_science::equals(true)])],
            vec![client
                .interests()
                .count(vec![interests::coding::equals(true)])],
        ))
        .await
        .unwrap();
    HttpResponse::Ok().body(format!("{{ \"webDevelopment\": {:?},\"cyberSecurity\": {:?},\"mobileDevelopment\": {:?},\"design\": {:?},\"dataScience\": {:?},\"coding\": {:?} }}",num_web_dev[0].to_owned(), num_cyber_sec[0].to_owned(), num_mobile_dev[0].to_owned(), num_design[0].to_owned(), num_data_science[0].to_owned(), num_coding[0].to_owned()))
}

pub async fn users_between_dates(start: Path<String>, end: Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "start: {}, end: {}",
        start.to_owned(),
        end.to_owned()
    ))

#[post("/api/updateUser")]
pub async fn update_user(updatedUser: Json<DbUser>) -> HttpResponse {
    let client = prisma::new_client().await.unwrap();
    let updated_user_data = client
        .user()
        .update(
            user::id::equals(updatedUser.id),
            vec![
                user::first_name::set(updatedUser.firstName.to_owned()),
                user::last_name::set(updatedUser.lastName.to_owned()),
                user::mail::set(updatedUser.mail.to_owned()),
                user::picture::set(updatedUser.picture.to_owned()),
            ],
        )
        .exec()
        .await;
    match updated_user_data {
        Ok(_) => {
            let updated_interests = client
                .interests()
                .upsert(
                    interests::user_id::equals(updatedUser.id),
                    interests::create(
                        updatedUser.interests.webDevelopment.to_owned(),
                        updatedUser.interests.cyberSecurity.to_owned(),
                        updatedUser.interests.mobileDev.to_owned(),
                        updatedUser.interests.design.to_owned(),
                        updatedUser.interests.dataScience.to_owned(),
                        updatedUser.interests.coding.to_owned(),
                        user::id::equals(updatedUser.id),
                        vec![],
                    ),
                    vec![
                        interests::web_development::set(updatedUser.interests.webDevelopment),
                        interests::cyber_security::set(updatedUser.interests.webDevelopment),
                        interests::mobile_dev::set(updatedUser.interests.webDevelopment),
                        interests::design::set(updatedUser.interests.webDevelopment),
                        interests::data_science::set(updatedUser.interests.webDevelopment),
                        interests::coding::set(updatedUser.interests.webDevelopment),
                    ],
                )
                .exec()
                .await;
            match updated_interests {
                Err(err) => HttpResponse::NotModified()
                    .body(format!("user interests could not be updated: {}", err)),
                Ok(_) => {
                    let updated_company = client
                        .company_data()
                        .upsert(
                            company_data::user_id::equals(updatedUser.id),
                            company_data::create(
                                updatedUser.company.isAssociated.to_owned(),
                                updatedUser.company.companyName.to_owned(),
                                updatedUser.company.companyEmail.to_owned(),
                                user::id::equals(updatedUser.id),
                                vec![],
                            ),
                            vec![
                                company_data::is_associated::set(
                                    updatedUser.company.isAssociated.to_owned(),
                                ),
                                company_data::company_name::set(
                                    updatedUser.company.companyName.to_owned(),
                                ),
                                company_data::company_email::set(
                                    updatedUser.company.companyEmail.to_owned(),
                                ),
                            ],
                        )
                        .exec()
                        .await;
                    match updated_company {
                        Ok(_) => {
                            return HttpResponse::Ok().body("done");
                        }
                        Err(err) => HttpResponse::NotModified().body(format!(
                            "User could not be modified because of the following error: {}",
                            err
                        )),
                    }
                }
            }
        }
        Err(err) => {
            HttpResponse::NotModified().body(format!("user data could no be updated: {}", err))
        }
    }

}
