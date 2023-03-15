use actix_web::{guard, web, App, HttpServer};
mod ws;
use ws::index;
mod routes;
use routes::admin::authenticate_admin;
use routes::helper::generat_data::generate_data;
use routes::subscribtion::{subscribe, unsubscribe};
use routes::user_related::counts::{num_of_interest, number_of_associates, number_of_users};
use routes::user_related::{
    create_new_user, delete_user, get_all_users, get_specific_user, users_between_dates,
};
#[tokio::main]
//test test eteest
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .guard(guard::Header("Admin", "true"))
                    .route("/getAllUsers", web::get().to(get_all_users))
                    .route(
                        "/getSpecificUser/{user_mail}",
                        web::get().to(get_specific_user),
                    )
                    .route("/generateData", web::post().to(generate_data))
                    .route("/deleteUser/{user_id}", web::delete().to(delete_user))
                    .route("/numOfUsers", web::get().to(number_of_users))
                    .route("/numOfAssociates", web::get().to(number_of_associates))
                    .route("/numOfInterests", web::get().to(num_of_interest))
                    .route(
                        "/usersBetweenDates/{start}-{end}",
                        web::get().to(users_between_dates),
                    ),
            )
            .service(
                web::scope("/api")
                    .route("/createUser", web::post().to(create_new_user))
                    .route("/login", web::post().to(authenticate_admin))
                    .route("/subscribe/{adress}", web::put().to(subscribe))
                    .route("/unsubscribe/{adress}", web::delete().to(unsubscribe)),
            )
            .service(web::scope("/ws").route("/", web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
