use lambda_runtime::{service_fn, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {}

#[derive(Debug, Serialize)]
struct Response {
    body: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id))]
async fn hello_world(event: LambdaEvent<Request>) -> Result<Response, anyhow::Error> {
    Ok(Response {
        body: r#"{"message": "Hello, world"}"#.into(),
    })
}
