use aws_lambda_events::s3::S3Event;
use aws_sdk_rekognition::types::{Image, S3Object};
use futures::{stream, StreamExt};
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};

use crate::common::Common;

#[derive(Deserialize, Serialize)]
pub struct Request(S3Event);

#[derive(Debug, Serialize)]
pub struct Response {}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

async fn detect_record(
    common: &Common,
    record: &aws_lambda_events::s3::S3EventRecord,
) -> Result<(), anyhow::Error> {
    let bucket = record.s3.bucket.name.as_ref().expect("got bucket name");
    let object = record.s3.object.key.as_ref().expect("object has key");
    let image = Image::builder()
        .s3_object(S3Object::builder().bucket(bucket).name(object).build())
        .build();
    let labels = common
        .rekognition_client()
        .detect_labels()
        .image(image)
        .max_labels(10)
        .send()
        .await?;

    for label in labels.labels().expect("found labels") {
        let _ = common
            .dynamodb_client()
            .update_item()
            .key(
                "Label",
                aws_sdk_dynamodb::types::AttributeValue::S(
                    label.name().expect("found label name").to_string(),
                ),
            )
            // Using an update expression ensures the count and list are updated atomically.
            // This does require passing `:one` as a value.
            .update_expression("SET count = count + :one SET images = images + :image")
            .expression_attribute_values(
                ":one",
                aws_sdk_dynamodb::types::AttributeValue::N("1".to_string()),
            )
            .expression_attribute_values(
                ":image",
                aws_sdk_dynamodb::types::AttributeValue::S(object.to_string()),
            )
            .send()
            .await?;
    }

    Ok(())
}

#[tracing::instrument(skip(common, event), fields(req_id = %event.context.request_id, record_count = event.payload.0.records.len()))]
pub async fn handler(
    common: &Common,
    event: LambdaEvent<Request>,
) -> Result<Response, anyhow::Error> {
    let count = stream::iter(event.payload.0.records)
        .map(|r| async move { detect_record(common, &r).await })
        .buffered(1)
        .count()
        .await;

    tracing::trace!("Handled {count} records");

    Ok(Response {})
}
