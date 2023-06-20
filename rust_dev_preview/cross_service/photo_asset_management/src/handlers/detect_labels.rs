use std::collections::HashMap;

use anyhow::anyhow;
use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    s3::{S3Event, S3EventRecord},
};
use aws_sdk_dynamodb::operation::update_item::builders::UpdateItemFluentBuilder;
use aws_sdk_rekognition::types::{Image, Label, S3Object};
use lambda_runtime::LambdaEvent;

use crate::{apig_response, common::Common};

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
        // Using an update expression ensures that the count and list are updated atomically.
        // This does require passing `:one` as a value.
        .update_expression("SET #Count = if_not_exists(#Count, :zero) + :one, Images = list_append(if_not_exists(Images, :empty), :image)")
        .expression_attribute_names("#Count", "Count")
        .expression_attribute_values(
            ":zero",
            aws_sdk_dynamodb::types::AttributeValue::N("0".to_string()),
        )
        .expression_attribute_values(
            ":one",
            aws_sdk_dynamodb::types::AttributeValue::N("1".to_string()),
        )
        .expression_attribute_values(
            ":image",
            aws_sdk_dynamodb::types::AttributeValue::L(vec![
                aws_sdk_dynamodb::types::AttributeValue::S(object.to_string()),
            ]),
        )
        .expression_attribute_values(
            ":empty",
            aws_sdk_dynamodb::types::AttributeValue::L(vec![ ]),
        )
}

async fn detect_record<'a>(
    common: &Common,
    bucket: &String,
    object: &String,
) -> Result<Vec<Label>, anyhow::Error> {
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
        .await?
        .labels()
        .ok_or_else(|| anyhow!("missing labels"))?
        .iter()
        .map(|l| l.to_owned())
        .collect();

    Ok(labels)
}

pub async fn find_labels(
    common: &Common,
    records: Vec<S3EventRecord>,
) -> Result<HashMap<String, Vec<Label>>, anyhow::Error> {
    let mut object_labels_map = HashMap::<String, Vec<Label>>::with_capacity(records.len());

    for record in records {
        let object = record
            .s3
            .object
            .key
            .ok_or_else(|| anyhow!("missing object key"))?;
        let labels = detect_record(common, common.storage_bucket(), &object).await?;
        object_labels_map.insert(object, labels);
    }

    Ok(object_labels_map)
}

pub async fn apply_updates(
    common: &Common,
    updates: HashMap<String, Vec<Label>>,
) -> Result<usize, anyhow::Error> {
    let mut count = 0;
    for (object, labels) in updates {
        tracing::info!(object, ?labels, "Adding labels for image");
        for label in labels {
            let update = common
                .dynamodb_client()
                .update_item()
                .table_name(common.labels_table());
            let expression = prepare_update_expression(update, &object, &label);
            let result = expression.send().await?;
            tracing::info!(?result, "Updated image with labels");
        }
        count += 1;
    }

    Ok(count)
}

#[tracing::instrument(skip(common, request))]
pub async fn handler(
    common: &Common,
    request: LambdaEvent<S3Event>,
) -> Result<ApiGatewayProxyResponse, anyhow::Error> {
    let updates = find_labels(common, request.payload.records).await?;
    let count = apply_updates(common, updates).await?;

    tracing::trace!("Handled {count} records");
    Ok(apig_response!(format!("Handled {count} records")))
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::prepare_update_expression;
    use aws_config::SdkConfig;
    use aws_sdk_dynamodb::types::AttributeValue;

    #[tokio::test]
    async fn test_prepare_update_statement() {
        let object = "object".to_string();
        let label = aws_sdk_rekognition::types::Label::builder()
            .name("label")
            .build();

        let client = aws_sdk_dynamodb::Client::new(&SdkConfig::builder().build());
        let update = client.update_item();
        let update = prepare_update_expression(update, &object, &label);

        assert!(update.get_table_name().is_none());
        assert_eq!(
            *update.get_key(),
            Some(HashMap::from([(
                "Label".to_string(),
                AttributeValue::S("label".to_string())
            )]))
        );
        assert_eq!(
            *update.get_update_expression(),
            Some(String::from("SET #Count = if_not_exists(#Count, :zero) + :one, Images = list_append(if_not_exists(Images, :empty), :image)"))
        );
        assert_eq!(
            *update.get_expression_attribute_names(),
            Some(HashMap::from([("#Count".to_string(), "Count".to_string())])),
        );

        // Example using the inner directly
        let update_inner = update.inner();
        assert_eq!(
            *update_inner.get_expression_attribute_values(),
            Some(HashMap::from([
                (":empty".to_string(), AttributeValue::L(vec![])),
                (
                    ":image".to_string(),
                    AttributeValue::L(vec![AttributeValue::S("object".to_string())]),
                ),
                (":one".to_string(), AttributeValue::N("1".to_string())),
                (":zero".to_string(), AttributeValue::N("0".to_string())),
            ])),
        );
    }
}
