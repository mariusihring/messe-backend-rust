pub mod prisma;
mod structs;
use actix_web::{get, post, put, web::Json, web::Path, HttpResponse, Responder};
use prisma::{company_data, interests, user};
use std::fs;
use structs::{DbUser, NewUser, Person};

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

#[get("/api/getSpecificUser/{user_mail}")]
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
                format!("{}@{}.com", user.name, user.family_name),
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
pub async fn create_new_user(user: Json<NewUser>) -> HttpResponse {
    let client = prisma::new_client().await.unwrap();

    let data = client
        .user()
        .create(
            user.lastName.to_owned(),
            user.firstName.to_owned(),
            user.mail.to_owned(),
            user.picture.to_owned(),
            vec![],
        )
        .exec()
        .await;

    match data {
        Err(_) => {
            return HttpResponse::Conflict()
                .body("User allready exists, please use a different mail")
        }
        Ok(_) => {
            let id = client
                .user()
                .find_first(vec![user::mail::equals(user.mail.to_string())])
                .exec()
                .await;
            match id {
                Err(_) => todo!(),
                Ok(data) => match data {
                    Some(db_data) => {
                        let interests = client
                            .interests()
                            .create(
                                user.interests.webDevelopment,
                                user.interests.cyberSecurity,
                                user.interests.mobileDev,
                                user.interests.design,
                                user.interests.dataScience,
                                user.interests.coding,
                                user::id::equals(db_data.id),
                                vec![],
                            )
                            .exec()
                            .await;
                        match interests {
                            Err(err) => {
                                return HttpResponse::NotModified()
                                    .body(format!("Error creating Interests: {}", err))
                            }
                            Ok(_) => {
                                let company = client
                                    .company_data()
                                    .create(
                                        user.company.isAssociated,
                                        user.company.companyEmail.to_owned(),
                                        user.company.companyName.to_owned(),
                                        user::id::equals(db_data.id),
                                        vec![],
                                    )
                                    .exec()
                                    .await;

                                match company {
                                    Err(err) => {
                                        return HttpResponse::NotModified()
                                            .body(format!("Error creating Company Data: {}", err))
                                    }
                                    Ok(_) => {
                                        return HttpResponse::Ok().body("Succesfully created User")
                                    }
                                };
                            }
                        }
                    }
                    None => {
                        return HttpResponse::NotModified()
                            .body("Error retreaving userid from db to create Relations")
                    }
                },
            }
        }
    }
}
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
                            vec![],
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
