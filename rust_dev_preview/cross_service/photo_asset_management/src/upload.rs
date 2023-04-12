use lambda_runtime::{service_fn, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct Request {
    file_name: String,
}

#[derive(Debug, Serialize)]
struct Response {
    body: String,
}

#[derive(Serialize)]
struct Url {
    url: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

async fn make_url(file_name: String) -> Result<Url, anyhow::Error> {
    let url = format!("https://s3-presigned/{}?...", file_name);
    Ok(Url { url })
}

#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id))]
async fn upload(event: LambdaEvent<Request>) -> Result<Response, anyhow::Error> {
    let url = make_url(event.payload.file_name).await?;

    Ok(Response {
        body: json!(url).to_string(),
    })
}
