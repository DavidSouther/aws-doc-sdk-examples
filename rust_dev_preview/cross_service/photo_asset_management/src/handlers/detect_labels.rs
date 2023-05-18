use anyhow::anyhow;
use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use aws_lambda_events::s3::S3Event;
use aws_sdk_dynamodb::operation::update_item::builders::UpdateItemFluentBuilder;
use aws_sdk_rekognition::types::{Image, Label, S3Object};
use futures::{stream, StreamExt};
use lambda_runtime::LambdaEvent;
use serde::Serialize;

use crate::common::Common;

pub struct DetectLabelsRequest(S3Event);

fn prepare_update_expression(
    update: UpdateItemFluentBuilder,
    object: &String,
    label: &Label,
) -> UpdateItemFluentBuilder {
    update
        .key(
            "Label",
            aws_sdk_dynamodb::types::AttributeValue::S(
                label.name().expect("found label name").to_string(),
            ),
        )
        // Using an update expression ensures the count and list are updated atomically.
        // This does require passing `:one` as a value.
        .update_expression("SET Count = Count + :one SET Images = Images + :image")
        .expression_attribute_values(
            ":one",
            aws_sdk_dynamodb::types::AttributeValue::N("1".to_string()),
        )
        .expression_attribute_values(
            ":image",
            aws_sdk_dynamodb::types::AttributeValue::S(object.to_string()),
        )
}

async fn detect_record(
    common: &Common,
    record: &aws_lambda_events::s3::S3EventRecord,
) -> Result<(), anyhow::Error> {
    let bucket = record
        .s3
        .bucket
        .name
        .as_ref()
        .ok_or_else(|| anyhow!("missing bucket name"))?;
    let object = record
        .s3
        .object
        .key
        .as_ref()
        .ok_or_else(|| anyhow!("missing object key"))?;

    let labels = common
        .rekognition_client()
        .detect_labels()
        .image(
            Image::builder()
                .s3_object(S3Object::builder().bucket(bucket).name(object).build())
                .build(),
        )
        .max_labels(10)
        .send()
        .await?;

    for label in labels.labels().expect("found labels") {
        // let update = prepare_update_expression(object, label)?;
        // common.dynamodb_client().update_item().
        let update = common.dynamodb_client().update_item();
        let _ = prepare_update_expression(update, object, label)
            .send()
            .await?;
    }

    Ok(())
}

#[tracing::instrument(skip(common, request))]
pub async fn handler(
    common: &Common,
    request: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<impl Serialize, anyhow::Error> {
    let body = request
        .payload
        .body
        .ok_or_else(|| anyhow!("missing s3 event body"))?;
    let event: S3Event = serde_json::from_str(body.as_str())?;
    let count = stream::iter(event.records)
        .map(|r| async move { detect_record(common, &r).await })
        .buffered(1)
        .count()
        .await;

    tracing::trace!("Handled {count} records");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::prepare_update_expression;
    use aws_config::SdkConfig;

    #[test]
    fn test_prepare_update_statement() {
        let object = "object".to_string();
        let label = aws_sdk_rekognition::types::Label::builder()
            .name("label")
            .build();

        let client = aws_sdk_dynamodb::Client::new(&SdkConfig::builder().build());
        let update = client.update_item();
        let update = prepare_update_expression(update, &object, &label);

        // TODO: This test would be better if it could get an UpdateItemInput directly, but that's
        // hidden inside the SDK. Waiting for smithy-rs to expose it more directly.
        let update_debug = format!("{:?}", update);
        let split = update_debug
            .split(", inner: UpdateItemInputBuilder ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let update_inner_debug = split.get(1).expect("inner as Debug");
        assert_eq!(
            update_inner_debug,
            r#"{ table_name: None, key: Some({"Label": S("label")}), attribute_updates: None, expected: None, conditional_operator: None, return_values: None, return_consumed_capacity: None, return_item_collection_metrics: None, update_expression: Some("SET Count = Count + :one SET Images = Images + :image"), condition_expression: None, expression_attribute_names: None, expression_attribute_values: Some({":image": S("object"), ":one": N("1")}) } }"#
        );
    }
}
