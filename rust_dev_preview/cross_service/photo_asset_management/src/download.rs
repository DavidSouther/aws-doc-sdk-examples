use std::collections::HashSet;

use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use futures::{stream, StreamExt};
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::Common;

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

#[pin_project::pin_project]
struct ZipBody<'pin, InnerBody> {
    #[pin]
    inner: InnerBody,
    images: Box<dyn Iterator<Item = String>>,
    s3_client: &'pin aws_sdk_s3::Client,
    // image: Option<&'pin >,
    upload_id: usize,
}

async fn do_download(
    common: &Common,
    labels: Vec<String>,
    _notify: String,
) -> Result<(), anyhow::Error> {
    let count = labels.len();
    let _images: HashSet<String> = stream::iter(labels)
        .map(move |label| async { get_images(common, label).await.expect("got images") })
        .buffered(count)
        .collect::<Vec<_>>()
        .await
        .iter()
        .flatten()
        .cloned()
        .collect();

    // let flat = par_iter.flat_map_iter(map_op)

    // let images: HashSet<String> =
    //     .collect()
    //     .fold();

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
