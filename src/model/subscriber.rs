use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use bambangshop::REQUEST_CLIENT;
use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    pub async fn update(&self, payload: Notification) {
        REQUEST_CLIENT
            .post(&self.url)
            .header("Content-Type", "application/json")
            .body(to_string(&payload).unwrap())
            .send()
            .await
            .ok();

        log::warn!(
            "[📢] Notification of: [{}] {} to: {}",
            payload.status, payload.product_type, self.url
        );
    }
}
