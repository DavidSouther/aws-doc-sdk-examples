use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use lambda_runtime::LambdaEvent;
use serde::Serialize;

use crate::common::Common;

#[tracing::instrument(skip(_common))]
pub async fn handler(
    _common: &Common,
    request: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<impl Serialize, anyhow::Error> {
    Ok(format!(
        "Hello, {:?}",
        request.payload.body.unwrap_or_default()
    ))
}
