use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

pub mod routes;
use routes::{create_new_user, get_all_users, get_specific_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all_users)
            .service(get_specific_user)
            .service(create_new_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
