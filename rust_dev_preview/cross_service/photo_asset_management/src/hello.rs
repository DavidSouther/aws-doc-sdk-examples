use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};

use crate::common::Common;

#[derive(Deserialize)]
pub struct Request {}

#[derive(Debug, Serialize)]
pub struct Response {
    body: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

#[tracing::instrument(skip(_common, event), fields(req_id = %event.context.request_id))]
pub async fn handler(
    _common: &Common,
    event: LambdaEvent<Request>,
) -> Result<Response, anyhow::Error> {
    Ok(Response {
        body: r#"{"message": "Hello, world"}"#.into(),
    })
}
