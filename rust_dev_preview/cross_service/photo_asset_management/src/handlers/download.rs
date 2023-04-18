use std::collections::HashSet;

use crate::{common::Common, uploader::ZipUpload};
use aws_lambda_events::apigw::ApiGatewayProxyRequest;

use aws_sdk_s3::presigning::PresigningConfig;
use chrono::Duration;
use futures::{stream, StreamExt};
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

async fn get_images_for_labels(labels: Vec<String>, common: &Common) -> HashSet<String> {
    let count = labels.len();
    stream::iter(labels)
        .map(move |label| async {
            get_images_for_label(common, label)
                .await
                .expect("got images")
        })
        .buffered(count)
        .collect::<Vec<_>>()
        .await
        .iter()
        .flatten()
        .cloned()
        .collect::<HashSet<String>>()
}

async fn get_images_for_label(
    common: &Common,
    label: String,
) -> Result<Vec<String>, anyhow::Error> {
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

async fn send_notification(
    common: &Common,
    destination: (String, String),
) -> Result<(), anyhow::Error> {
    let get_object = common
        .s3_client()
        .get_object()
        .bucket(destination.0)
        .key(destination.1)
        .presigned(
            PresigningConfig::builder()
                .expires_in(Duration::days(1).to_std()?)
                .build()?,
        )
        .await?;
    let message = format!("Retrieve your photos {}", get_object.uri());
    common
        .sns_client()
        .publish()
        .topic_arn(common.notification_arn())
        .message(message)
        .send()
        .await?;
    Ok(())
}

async fn do_download(common: &Common, labels: Vec<String>) -> Result<(), anyhow::Error> {
    let images = get_images_for_labels(labels, common).await;

    let mut zip_upload = ZipUpload::builder(common).build().await?;

    for image in images {
        zip_upload.add_object(image).await?;
    }

    let destination = zip_upload.finish().await?;

    send_notification(common, destination).await?;

    Ok(())
}

#[tracing::instrument(skip(common, event), fields(req_id = %event.context.request_id))]
pub async fn handler(
    common: &Common,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<(), anyhow::Error> {
    let body = event.payload.body.expect("proxy request has a body");
    let request: Request = serde_json::from_str(&body).expect("body is valid download request");
    do_download(common, request.labels).await
}
