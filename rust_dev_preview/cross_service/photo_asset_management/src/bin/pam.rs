#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let handler = match env::var("_HANDLER").is_ok() {};

    lambda_runtime::run(service_fn(|event: LambdaEvent<Request>| async {
        handler(event).await
    }))
    .await
}
