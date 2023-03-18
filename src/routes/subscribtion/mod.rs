use crate::routes::prisma::{self, subscriber};
use actix_web::{web::Path, HttpResponse, Responder};
pub mod service;
pub async fn subscribe(adress: Path<String>) -> impl Responder {
    let client = prisma::new_client().await.unwrap();

    let sub = client
        .subscriber()
        .create(adress.to_owned(), vec![])
        .exec()
        .await
        .unwrap();
    HttpResponse::Ok().body(format!("Subscriber with id: {} has been added", sub.id))
}

pub async fn unsubscribe(adress: Path<String>) -> impl Responder {
    let client = prisma::new_client().await.unwrap();
    let adress = adress.to_owned();

    let _deleted_sub = client
        .subscriber()
        .find_first(vec![subscriber::adress::equals(adress.to_owned())])
        .exec()
        .await
        .unwrap();
    HttpResponse::Ok().body(format!("User with adress: {} has been removed", adress))
}
