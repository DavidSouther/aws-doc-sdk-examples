use aws_lambda_events::s3::S3Event;
use lambda_runtime::{service_fn, LambdaEvent};

mod detect_labels;
mod download;
mod hello;
mod labels;
mod upload;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let handler = std::env::var("_HANDLER").expect("_HANDLER provided");

    match handler.as_str() {
        "detect_labels::handler" => {
            lambda_runtime::run(service_fn(|event: LambdaEvent<S3Event>| async {
                detect_labels::handler(event).await
            }))
            .await
        }
        "download::handler" => {
            lambda_runtime::run(service_fn(|event: LambdaEvent<download::Request>| async {
                download::handler(event).await
            }))
            .await
        }
        "labels::handler" => {
            lambda_runtime::run(service_fn(|event: LambdaEvent<labels::Request>| async {
                labels::handler(event).await
            }))
            .await
        }
        "hello::handler" => {
            lambda_runtime::run(service_fn(|event: LambdaEvent<hello::Request>| async {
                hello::handler(event).await
            }))
            .await
        }
        "upload::handler" => {
            lambda_runtime::run(service_fn(|event: LambdaEvent<upload::Request>| async {
                upload::handler(event).await
            }))
            .await
        }
        s => panic!("Missing handler for {s}"),
    }
}
