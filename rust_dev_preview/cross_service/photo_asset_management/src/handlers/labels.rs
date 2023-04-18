use std::collections::HashMap;

use aws_lambda_events::apigw::ApiGatewayProxyResponse;
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{apig_response, common::Common};

#[derive(Deserialize)]
pub struct Request {}

#[derive(Serialize)]
pub struct Response {
    body: String,
}

#[derive(Serialize)]
struct Label {
    count: u32,
}

impl Label {
    fn new(count: u32) -> Self {
        Label { count }
    }
}

#[derive(Serialize)]
struct Labels {
    labels: HashMap<String, Label>,
}

impl Labels {
    fn new() -> Self {
        Labels {
            labels: HashMap::new(),
        }
    }

    fn add(&mut self, label: String, count: u32) {
        self.labels.insert(label, Label::new(count));
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

async fn get_labels(_common: &Common) -> Result<Labels, anyhow::Error> {
    let mut labels = Labels::new();
    labels.add("mountain".into(), 5);
    labels.add("lake".into(), 3);

    // TODO Get Labels

    Ok(labels)
}

#[tracing::instrument(skip(common, event), fields(req_id = %event.context.request_id))]
pub async fn handler(
    common: &Common,
    event: LambdaEvent<Request>,
) -> Result<ApiGatewayProxyResponse, anyhow::Error> {
    let labels = get_labels(common).await?;

    Ok(apig_response!(labels))
}
