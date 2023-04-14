use lambda_runtime::{service_fn, LambdaEvent};
use photo_asset_management::{common::Common, detect_labels};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let common = Common::init().await;

    lambda_runtime::run(service_fn(
        |event: LambdaEvent<detect_labels::Request>| async {
            detect_labels::handler(&common, event).await
        },
    ))
    .await
}
