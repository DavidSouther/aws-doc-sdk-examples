use anyhow::anyhow;
use lambda_runtime::{service_fn, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct Request {
    labels: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Response {
    body: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

async fn do_download(labels: Vec<String>, notify: String) -> Result<(), anyhow::Error> {
    Ok(())
}

async fn download(event: LambdaEvent<Request>) -> Result<(), anyhow::Error> {
    // let notify = .unwrap().identity_id;
    match event.context.identity {
        Some(identity) => {
            do_download(event.payload.labels, identity.identity_id).await?;
            Ok(())
        }
        None => Err(anyhow!("Missing cognito identity")),
    }
}
