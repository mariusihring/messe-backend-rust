use actix_web::{App, HttpServer};

pub mod routes;

use routes::{create_new_user, generate_data, get_all_users, get_specific_user, update_user};

#[tokio::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all_users)
            .service(get_specific_user)
            .service(create_new_user)
            .service(generate_data)
            .service(delete_user)
            .service(update_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
