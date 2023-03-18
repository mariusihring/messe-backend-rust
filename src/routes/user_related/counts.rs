use crate::routes::prisma::{self, company_data, interests};
use actix_web::{HttpResponse, Responder};
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
