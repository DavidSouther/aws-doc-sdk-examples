use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use lambda_runtime::{service_fn, LambdaEvent};
use photo_asset_management::{common::Common, download};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let common = Common::init().await;

    lambda_runtime::run(service_fn(
        |event: LambdaEvent<ApiGatewayProxyRequest>| async {
            download::handler(&common, event).await
        },
    ))
    .await
}
