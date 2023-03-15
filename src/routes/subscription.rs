use crate::routes::prisma;
use crate::routes::HttpResponse;
use websocket::{ClientBuilder, OwnedMessage};

use super::prisma::user;

pub async fn notify_subscribers() -> HttpResponse {
    let client = prisma::new_client().await.unwrap();
    let users = client
        .user()
        .find_many(vec![])
        .with(user::interests::fetch())
        .with(user::company_data::fetch())
        .exec()
        .await
        .unwrap();
    let subscribers = client.subscriber().find_many(vec![]).exec().await.unwrap();
    for sub in subscribers.iter() {
        let mut client = ClientBuilder::new(&sub.adress)
            .unwrap()
            .connect_insecure()
            .unwrap();
        let json = serde_json::to_string(&users).unwrap();
        let message = OwnedMessage::Text(json);
        client.send_message(&message).unwrap();
    }

    HttpResponse::Ok().body("Ok")
}
