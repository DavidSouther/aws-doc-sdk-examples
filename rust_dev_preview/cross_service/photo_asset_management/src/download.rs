use std::{collections::HashSet, mem};

use crate::common::Common;
use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use aws_sdk_dynamodb::primitives::DateTime;
use aws_sdk_s3::types::CompletedPart;
use aws_smithy_types_convert::date_time::DateTimeExt;
use chrono::NaiveDateTime;
use futures::{stream, StreamExt, TryStreamExt};
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};
use serde_json::json;
use streaming_zip::{Archive, CompressionMode};

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

async fn get_images(common: &Common, label: String) -> Result<Vec<String>, anyhow::Error> {
    let response = common
        .dynamodb_client()
        .get_item()
        .key(
            "Label",
            aws_sdk_dynamodb::types::AttributeValue::S(label.to_string()),
        )
        .attributes_to_get("Images")
        .send()
        .await?;
    let item = response.item().expect("got images for label");
    let images = item.get("Images").expect("has images");
    let attribute = images.as_ss().expect("images are an attribute set");
    Ok(attribute.clone())
}

async fn do_download(
    common: &Common,
    labels: Vec<String>,
    _notify: String,
) -> Result<(), anyhow::Error> {
    let count = labels.len();
    let images: HashSet<String> = stream::iter(labels)
        .map(move |label| async { get_images(common, label).await.expect("got images") })
        .buffered(count)
        .collect::<Vec<_>>()
        .await
        .iter()
        .flatten()
        .cloned()
        .collect();

    // let flat = par_iter.flat_map_iter(map_op)
    let key = uuid::Uuid::new_v4();
    let upload = common
        .s3_client()
        .create_multipart_upload()
        .bucket(common.working_bucket())
        .key(key.to_string())
        .send()
        .await?;

    let upload_id = upload
        .upload_id()
        .expect("can multipart upload")
        .to_string();

    let mut part = 0;
    let mut body_bytes: Vec<u8> = Vec::new();
    let mut zip_writer = Archive::new(body_bytes);
    let mut upload_parts: Vec<CompletedPart> = Vec::new();

    // let mut write_body_bytes =
    //     |part: &mut i32, bytes: &mut Vec<u8>, upload_parts: &mut Vec<CompletedPart>| async {
    async fn write_body_bytes(
        part: &mut i32,
        bytes: &mut Vec<u8>,
        upload_parts: &mut Vec<CompletedPart>,
        common: &Common,
        key: uuid::Uuid,
    ) -> Result<(), anyhow::Error> {
        let mut body: Vec<u8> = Vec::new();
        mem::swap(bytes, &mut body);
        let upload_part_response = common
            .s3_client()
            .upload_part()
            .bucket(common.working_bucket())
            .key(key.to_string())
            .body(body.into())
            .part_number(*part)
            .send()
            .await?;
        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_response.e_tag().unwrap_or_default())
                .part_number(*part)
                .build(),
        );
        *part += 1;
        Ok::<(), anyhow::Error>(())
    }

    for image in images {
        let mut object = common
            .s3_client()
            .get_object()
            .bucket(common.storage_bucket())
            .key(image.clone())
            .send()
            .await?;

        let last_modified: NaiveDateTime = object
            .last_modified
            .unwrap_or_else(|| DateTime::from_millis(0))
            .to_chrono_utc()
            .expect("converted to chrono")
            .naive_utc();

        let _ = zip_writer
            .start_new_file(
                image.into_bytes(),
                last_modified,
                CompressionMode::Deflate(8),
                false,
            )
            .expect("started new file");

        while let Some(bytes) = object.body.try_next().await? {
            zip_writer.append_data(&bytes);

            write_body_bytes(&mut part, &mut body_bytes, &mut upload_parts, common, key).await?;
        }

        let _ = zip_writer.finish_file().expect("closed up file");
        write_body_bytes(&mut part, &mut body_bytes, &mut upload_parts, common, key).await?;
    }

    zip_writer.finish().expect("finished stream");
    write_body_bytes(&mut part, &mut body_bytes, &mut upload_parts, common, key).await?;

    let _ = common.s3_client().complete_multipart_upload();

    // Send notification

    Ok(())
}

#[tracing::instrument(skip(common, event), fields(req_id = %event.context.request_id))]
pub async fn handler(
    common: &Common,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<(), anyhow::Error> {
    let body = event.payload.body.expect("proxy request has a body");
    let identity = event
        .context
        .identity
        .expect("cognito identity for request");
    let request: Request = serde_json::from_str(&body).expect("body is valid");
    do_download(common, request.labels, identity.identity_id).await
}
