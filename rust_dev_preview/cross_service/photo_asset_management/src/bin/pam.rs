use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use lambda_runtime::{service_fn, LambdaEvent};
use photo_asset_management::{
    common::Common,
    handlers::{detect_labels, download, hello, labels, upload},
};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let common = Common::init().await;

    let handler = std::env::var("_HANDLER").expect("_HANDLER provided");

    match handler.as_str() {
        "detect_labels::handler" => {
            lambda_runtime::run(service_fn(
                |event: LambdaEvent<detect_labels::Request>| async {
                    detect_labels::handler(&common, event).await
                },
            ))
            .await
        }
        "download::handler" => {
            lambda_runtime::run(service_fn(
                |event: LambdaEvent<ApiGatewayProxyRequest>| async {
                    download::handler(&common, event).await
                },
            ))
            .await
        }
        "labels::handler" => {
            lambda_runtime::run(service_fn(|event: LambdaEvent<labels::Request>| async {
                labels::handler(&common, event).await
            }))
            .await
        }
        "hello::handler" => {
            lambda_runtime::run(service_fn(|event: LambdaEvent<hello::Request>| async {
                hello::handler(&common, event).await
            }))
            .await
        }
        "upload::handler" => {
            lambda_runtime::run(service_fn(|event: LambdaEvent<upload::Request>| async {
                upload::handler(&common, event).await
            }))
            .await
        }
        s => panic!("Missing handler for {s}"),
    }
}
