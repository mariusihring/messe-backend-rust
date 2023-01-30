mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("{} Marius", req_body))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
