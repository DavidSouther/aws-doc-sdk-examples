use std::{collections::HashSet, io::Read};

use crate::common::Common;
use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use aws_sdk_dynamodb::primitives::DateTime;
use aws_sdk_s3::{
    operation::get_object::GetObjectOutput, presigning::PresigningConfig, types::CompletedPart,
};
use aws_smithy_types_convert::date_time::DateTimeExt;
use chrono::{Duration, NaiveDateTime};
use futures::{stream, StreamExt, TryStreamExt};
use lambda_runtime::LambdaEvent;
use pipe::{PipeReader, PipeWriter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use streaming_zip::{Archive, CompressionMode};

#[derive(Deserialize)]
pub struct Request {
    labels: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    body: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

struct Uploader<'a> {
    part: i32,
    pipe: PipeReader,
    zip_writer: Archive<PipeWriter>,
    upload_parts: Vec<CompletedPart>,
    upload_id: String,
    key: uuid::Uuid,
    common: &'a Common,
}

impl<'a> Uploader<'a> {
    async fn build(common: &'a Common) -> Result<Uploader<'a>, anyhow::Error> {
        let part = 0;
        let pipe = pipe::pipe();
        let zip_writer = Archive::new(pipe.1);
        let upload_parts: Vec<CompletedPart> = Vec::new();
        let key = uuid::Uuid::new_v4();

        let upload = common
            .s3_client()
            .create_multipart_upload()
            .bucket(common.working_bucket())
            .key(key.to_string())
            .send()
            .await?;

        let upload_id = upload
            .upload_id()
            .expect("can multipart upload")
            .to_string();

        Ok(Uploader {
            part,
            pipe: pipe.0,
            zip_writer,
            upload_parts,
            common,
            key,
            upload_id,
        })
    }

    async fn write_body_bytes(&mut self) -> Result<(), anyhow::Error> {
        let mut body = [0u8; 65356];
        let read = self.pipe.read(&mut body)?;
        tracing::trace!(read, "Read zipped bytes");
        let upload_part_response = self
            .common
            .s3_client()
            .upload_part()
            .bucket(self.common.working_bucket())
            .key(self.key.to_string())
            .body(Vec::from(body).into())
            .part_number(self.part)
            .upload_id(self.upload_id.clone())
            .send()
            .await?;
        self.upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_response.e_tag().unwrap_or_default())
                .part_number(self.part)
                .build(),
        );
        self.part += 1;
        Ok::<(), anyhow::Error>(())
    }

    async fn next_image(&mut self, image: String) -> Result<GetObjectOutput, anyhow::Error> {
        let object = self
            .common
            .s3_client()
            .get_object()
            .bucket(self.common.storage_bucket())
            .key(image.clone())
            .send()
            .await?;

        let last_modified: NaiveDateTime = object
            .last_modified
            .unwrap_or_else(|| DateTime::from_millis(0))
            .to_chrono_utc()
            .expect("converted to chrono")
            .naive_utc();

        self.zip_writer
            .start_new_file(
                image.into_bytes(),
                last_modified,
                CompressionMode::Deflate(8),
                false,
            )
            .expect("started new file");

        Ok(object)
    }

    async fn next_part(&mut self, bytes: &bytes::Bytes) -> Result<(), anyhow::Error> {
        self.zip_writer.append_data(bytes)?;

        self.write_body_bytes().await?;

        Ok(())
    }

    async fn finish_image(&mut self) -> Result<(), anyhow::Error> {
        self.zip_writer.finish_file()?;
        self.write_body_bytes().await?;
        Ok(())
    }

    async fn finish(mut self) -> Result<String, anyhow::Error> {
        let mut zip_writer = Archive::new(pipe::pipe().1);
        std::mem::swap(&mut self.zip_writer, &mut zip_writer);
        zip_writer.finish()?;
        self.write_body_bytes().await?;

        let _ = self
            .common
            .s3_client()
            .complete_multipart_upload()
            .bucket(self.common.working_bucket())
            .key(self.key.to_string())
            .upload_id(self.upload_id.clone())
            .send()
            .await?;
        Ok(self.key.to_string())
    }
}

async fn get_images(common: &Common, label: String) -> Result<Vec<String>, anyhow::Error> {
    let response = common
        .dynamodb_client()
        .get_item()
        .key(
            "Label",
            aws_sdk_dynamodb::types::AttributeValue::S(label.to_string()),
        )
        .attributes_to_get("Images")
        .send()
        .await?;
    let item = response.item().expect("got images for label");
    let images = item.get("Images").expect("has images");
    let attribute = images.as_ss().expect("images are an attribute set");
    Ok(attribute.clone())
}

async fn do_download(
    common: &Common,
    labels: Vec<String>,
    _notify: String,
) -> Result<(), anyhow::Error> {
    let count = labels.len();
    let images: HashSet<String> = stream::iter(labels)
        .map(move |label| async { get_images(common, label).await.expect("got images") })
        .buffered(count)
        .collect::<Vec<_>>()
        .await
        .iter()
        .flatten()
        .cloned()
        .collect();

    let mut uploader = Uploader::build(common).await?;

    for image in images {
        let mut object = uploader.next_image(image).await?;
        while let Some(bytes) = object.body.try_next().await? {
            uploader.next_part(&bytes).await?;
        }
        uploader.finish_image().await?;
    }

    let key = uploader.finish().await?;

    // Send notification
    let get_object = common
        .s3_client()
        .get_object()
        .bucket(common.working_bucket())
        .key(key.to_string())
        .presigned(
            PresigningConfig::builder()
                .expires_in(Duration::days(1).to_std()?)
                .build()?,
        )
        .await?;
    let message = format!("Retrieve your photos {}", get_object.uri());

    common
        .sns_client()
        .publish()
        .topic_arn(common.notification_arn())
        .message(message)
        .send()
        .await?;

    Ok(())
}

#[tracing::instrument(skip(common, event), fields(req_id = %event.context.request_id))]
pub async fn handler(
    common: &Common,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<(), anyhow::Error> {
    let body = event.payload.body.expect("proxy request has a body");
    let identity = event
        .context
        .identity
        .expect("cognito identity for request");
    let request: Request = serde_json::from_str(&body).expect("body is valid");
    do_download(common, request.labels, identity.identity_id).await
}
