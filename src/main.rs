use actix_web::{web, App, HttpServer};

pub mod routes;
use routes::{
    create_new_user, delete_user, generate_data, get_all_users, get_specific_user, num_of_interest,
    number_of_associates, number_of_users, users_between_dates,
};
#[tokio::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/getAllUsers", web::get().to(get_all_users))
                .route(
                    "/getSpecificUser/{user_mail}",
                    web::get().to(get_specific_user),
                )
                .route("/generateData", web::post().to(generate_data))
                .route("/createUser", web::post().to(create_new_user))
                .route("/deleteUser/{user_id}", web::delete().to(delete_user))
                .route("/numOfUsers", web::get().to(number_of_users))
                .route("/numOfAssociates", web::get().to(number_of_associates))
                .route("/numOfInterests", web::get().to(num_of_interest))
                .route(
                    "/usersBetweenDates/{start}-{end}",
                    web::get().to(users_between_dates),
                ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
