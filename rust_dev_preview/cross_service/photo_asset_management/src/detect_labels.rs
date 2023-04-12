use aws_lambda_events::s3::S3Event;
use lambda_runtime::LambdaEvent;
use rayon::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

async fn detect_record(
    _record: &aws_lambda_events::s3::S3EventRecord,
) -> Result<(), anyhow::Error> {
    Ok(())
}

#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id, record_count = event.payload.records.len()))]
pub async fn handler(event: LambdaEvent<S3Event>) -> Result<Response, anyhow::Error> {
    event
        .payload
        .records
        .par_iter()
        .map(|r| detect_record(r))
        .count();

    Ok(Response {})
}
