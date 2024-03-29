use crate::routes::prisma::{self, company_data, interests, user};
use crate::routes::structs::{DbUser, NewUser};
use crate::routes::subscribtion::service::notify_subscribers;
use actix_web::web::{Json, Path};
use actix_web::{HttpResponse, Responder};
pub mod counts;
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
    notify_subscribers().await;
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
    notify_subscribers().await;
    HttpResponse::Ok().body(format!{"User for mail {} successfully created with id {}", created_user.mail, created_user.id})
}

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
pub async fn users_between_dates(start: Path<String>, end: Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "start: {}, end: {}",
        start.to_owned(),
        end.to_owned()
    ))
}
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
