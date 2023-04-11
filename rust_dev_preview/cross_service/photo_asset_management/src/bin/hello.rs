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

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    lambda_runtime::run(service_fn(|event: LambdaEvent<Request>| async {
        hello_world(event).await
    }))
    .await
}
