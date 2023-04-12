use aws_lambda_events::s3::S3Event;
use rayon::prelude::*;
use std::fmt::Display;

use lambda_runtime::{service_fn, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Response {}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

async fn detect_record(record: &aws_lambda_events::s3::S3EventRecord) -> Result<(), anyhow::Error> {
    Ok(())
}

#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id, records = event.payload.records))]
async fn detect_labels(event: LambdaEvent<S3Event>) -> Result<Response, anyhow::Error> {
    event
        .payload
        .records
        .par_iter()
        .map(|r| detect_record(r))
        .collect()?;

    Ok(Response {})
}
