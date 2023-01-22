use std::env;

use pusher_beams::{PusherBeam, UsersPayload};
use serde_json::Value;


#[tokio::main]
async fn main() {
  let fcm_body = r#"{"notification":{"title":"Hello","body":"Hello, world!"}}"#;
    let users_payload = UsersPayload {
        users: vec!["user1".to_string()],
        apns: None,
        fcm: Some(fcm_body.parse().unwrap()),
        web: None,
    };
    println!("pushing to users");

    let instance_id = env::var("PUSHER_BEAMS_INSTANCE_ID").expect("PUSHER_BEAMS_INSTANCE_ID not set");
    let secret = env::var("PUSHER_BEAMS_SECRET").expect("PUSHER_BEAMS_SECRET not set");
    let pusher = PusherBeam::new(&instance_id, &secret);
    
    
    let result = pusher.publish_to_users(&users_payload).await.unwrap();
    println!("result: {:?}", result);
}
