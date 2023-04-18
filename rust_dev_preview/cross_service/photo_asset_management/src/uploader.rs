use std::io::Read;

use aws_sdk_dynamodb::primitives::DateTime;
use aws_sdk_s3::{operation::get_object::GetObjectOutput, types::CompletedPart};
use aws_smithy_types_convert::date_time::DateTimeExt;
use chrono::NaiveDateTime;
use futures::TryStreamExt;
use pipe::{PipeReader, PipeWriter};
use streaming_zip::{Archive, CompressionMode};
use uuid::Uuid;

use crate::common::Common;

// Pipe will read up to 10MB at a time. Each multipart upload will therefore also
// be in the 10MB range. Multipart uploads have a maximum part count of 10,000,
// so this imposes an effective limit on the size of the upload. Increasing this
// limit will use more memory but allow larger files. JPEGs typically 8MB, so this
// could be tuned but generally should allow ~10,000 images.
const READ_SIZE: usize = 1_048_578;

// ZipUploader is a struct to manage streaming a number of files into a single zip,
// that is itself streamed to an s3 object.
pub struct ZipUpload<'a> {
    part: i32,
    pipe: PipeReader,
    zip_writer: Archive<PipeWriter>,
    upload_parts: Vec<CompletedPart>,
    upload_id: String,
    key: String,
    dest_bucket: String,
    source_bucket: String,
    // common: &'a Common,
    s3_client: &'a aws_sdk_s3::Client,
}

pub struct ZipUploadBuilder<'a> {
    source_bucket: Option<String>,
    dest_bucket: Option<String>,
    key: Option<String>,
    common: &'a Common,
}

impl<'a> ZipUploadBuilder<'a> {
    pub fn key(&mut self, key: String) -> &Self {
        self.key = Some(key);
        self
    }

    pub fn source_bucket(&mut self, bucket: String) -> &Self {
        self.source_bucket = Some(bucket);
        self
    }

    pub fn dest_bucket(&mut self, bucket: String) -> &Self {
        self.dest_bucket = Some(bucket);
        self
    }

    pub async fn build(self) -> Result<ZipUpload<'a>, anyhow::Error> {
        let part = 0;
        let pipe = pipe::pipe();
        let zip_writer = Archive::new(pipe.1);
        let upload_parts: Vec<CompletedPart> = Vec::new();
        let key = self.key.unwrap_or_else(|| Uuid::new_v4().to_string());
        let source_bucket = self
            .source_bucket
            .unwrap_or_else(|| self.common.storage_bucket().clone());
        let dest_bucket = self
            .dest_bucket
            .unwrap_or_else(|| self.common.working_bucket().clone());

        let upload = self
            .common
            .s3_client()
            .create_multipart_upload()
            .bucket(&dest_bucket)
            .key(&key)
            .send()
            .await?;

        let upload_id = upload
            .upload_id()
            .expect("can multipart upload")
            .to_string();

        Ok(ZipUpload {
            part,
            pipe: pipe.0,
            zip_writer,
            upload_parts,
            // common: self.common,
            s3_client: self.common.s3_client(),
            key,
            dest_bucket,
            source_bucket,
            upload_id,
        })
    }
}

impl<'a> ZipUpload<'a> {
    pub fn builder(common: &'a Common) -> ZipUploadBuilder {
        ZipUploadBuilder {
            key: None,
            dest_bucket: None,
            source_bucket: None,
            common,
        }
    }

    // Read from the pipe until it's dry, and write those to the multipart upload
    // in READ_SIZE chunks.
    async fn write_body_bytes(&mut self) -> Result<(), anyhow::Error> {
        let mut body = [0u8; READ_SIZE];
        while self.pipe.read(&mut body)? > 0 {
            let upload_part_response = self
                .s3_client
                .upload_part()
                .bucket(&self.dest_bucket)
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
        }

        Ok(())
    }

    // Add an object in the
    pub async fn add_object(&mut self, key: String) -> Result<(), anyhow::Error> {
        let mut object = self.next_object(key).await?;
        while let Some(bytes) = object.body.try_next().await? {
            self.next_part(&bytes).await?;
        }
        self.finish_image().await?;

        Ok(())
    }

    // Move to the next object. This starts the object download from s3, and starts a new entry
    // in the Zip archive. It returns the GetObjectOutput to iterate the download's body.
    async fn next_object(&mut self, key: String) -> Result<GetObjectOutput, anyhow::Error> {
        let object = self
            .s3_client
            .get_object()
            .bucket(&self.source_bucket)
            .key(&key)
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
                key.into_bytes(),
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

    pub async fn finish(mut self) -> Result<(String, String), anyhow::Error> {
        let mut zip_writer = Archive::new(pipe::pipe().1);
        std::mem::swap(&mut self.zip_writer, &mut zip_writer);
        zip_writer.finish()?;
        self.write_body_bytes().await?;

        let _ = self
            .s3_client
            .complete_multipart_upload()
            .bucket(&self.dest_bucket)
            .key(&self.key)
            .upload_id(self.upload_id.clone())
            .send()
            .await?;

        Ok((self.dest_bucket, self.key))
    }
}
