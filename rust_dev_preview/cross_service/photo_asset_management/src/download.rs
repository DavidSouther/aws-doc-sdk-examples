use anyhow::anyhow;
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct Request {
    labels: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    body: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

async fn do_download(_labels: Vec<String>, _notify: String) -> Result<(), anyhow::Error> {
    Ok(())
}

#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id))]
pub async fn handler(event: LambdaEvent<Request>) -> Result<(), anyhow::Error> {
    // let notify = .unwrap().identity_id;
    match event.context.identity {
        Some(identity) => {
            do_download(event.payload.labels, identity.identity_id).await?;
            Ok(())
        }
        None => Err(anyhow!("Missing cognito identity")),
    }
}
