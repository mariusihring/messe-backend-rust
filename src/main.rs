use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

pub mod routes;
use routes::{get_all_users, get_specific_user, generate_data};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_all_users)
                                .service(get_specific_user)
                                .service(generate_data))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
