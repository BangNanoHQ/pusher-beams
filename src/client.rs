use reqwest::{header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Response};

use crate::{PublishResponse, UsersPayload};

pub struct PusherBeam {
    instance_id: String,
    secret_key: String,
    client: reqwest::Client,
}

impl PusherBeam {
    pub fn new(instance_id: &str, secret_key: &str) -> Self {
        Self {
            instance_id: instance_id.to_string(),
            secret_key: secret_key.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn publish_to_users(
        &self,
        payload: &UsersPayload,
    ) -> Result<PublishResponse, crate::Error> {
        let url = format!(
            "https://{}.pushnotifications.pusher.com/publish_api/v1/instances/{}/publishes/users",
            self.instance_id, self.instance_id
        );
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.secret_key))?,
        );
        headers.insert(
            CONTENT_TYPE,
            reqwest::header::HeaderValue::from_str("application/json")?,
        );
        headers.insert(
            ACCEPT,
            reqwest::header::HeaderValue::from_str("application/json")?,
        );

        let response: Response = self
            .client
            .post(&url)
            .headers(headers)
            .json(payload)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let result: PublishResponse = response.json().await?;
                Ok(result)
            }
            _ => {
                let body = response.text().await?.clone();
                Err(crate::Error::ResponseError(body))
            }
        }

        // Ok(())
    }
}
