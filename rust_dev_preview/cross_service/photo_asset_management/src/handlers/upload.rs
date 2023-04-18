use std::time::Duration;

use aws_lambda_events::apigw::ApiGatewayProxyResponse;
use aws_sdk_s3::presigning::PresigningConfig;
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{apig_response, common::Common};

#[derive(Deserialize)]
pub struct Request {
    file_name: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    body: String,
}

#[derive(Serialize)]
struct Url(String);

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

async fn make_put_url(common: &Common, file_name: String) -> Result<Url, anyhow::Error> {
    let uuid = uuid::Uuid::new_v4();
    let key_name = format!("{uuid}/{file_name}");
    let put_object = common
        .s3_client()
        .put_object()
        .bucket(common.storage_bucket())
        .key(key_name)
        .content_type("image/jpeg")
        .presigned(PresigningConfig::expires_in(Duration::from_secs(5 * 60))?)
        .await?;
    Ok(Url(put_object.uri().to_string()))
}

#[tracing::instrument(skip(common, event), fields(req_id = %event.context.request_id))]
pub async fn handler(
    common: &Common,
    event: LambdaEvent<Request>,
) -> Result<ApiGatewayProxyResponse, anyhow::Error> {
    let url = make_put_url(common, event.payload.file_name).await?;

    Ok(apig_response!(url))
}
