use serde::{Serialize, Deserialize};
use serde_json::{Value};


// payload for publishing to interests https://pusher.com/docs/beams/reference/publish-api/#publishing-a-notification-to-interests
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InterestsPayload {
  pub interests: Vec<String>, // maximum 100 interests, so you need to split this up if you have more than 100 interests
  pub webhook_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub apns: Option<Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fcm: Option<Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub web: Option<Value>,
}


// payload for publishing to users https://pusher.com/docs/beams/reference/publish-api/#publishing-a-notification-to-users
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UsersPayload {
    pub users: Vec<String>, // maximum 1000 users, so you need to split this up if you have more than 1000 users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublishResponse{
  publish_id: String,
}