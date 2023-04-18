use aws_config::SdkConfig;

pub struct Common {
    sdk_config: SdkConfig,
    rekognition_client: aws_sdk_rekognition::Client,
    dynamodb_client: aws_sdk_dynamodb::Client,
    s3_client: aws_sdk_s3::Client,
    sns_client: aws_sdk_sns::Client,
    storage_bucket: String,
    working_bucket: String,
    labels_table: String,
    notification_arn: String,
}

impl Common {
    pub async fn init() -> Self {
        let sdk_config = aws_config::load_from_env().await;
        let dynamodb_client = aws_sdk_dynamodb::Client::new(&sdk_config);
        let rekognition_client = aws_sdk_rekognition::Client::new(&sdk_config);
        let s3_client = aws_sdk_s3::Client::new(&sdk_config);
        let sns_client = aws_sdk_sns::Client::new(&sdk_config);
        Common {
            sdk_config,
            dynamodb_client,
            rekognition_client,
            s3_client,
            sns_client,
            storage_bucket: std::env::var("STORAGE_BUCKET").expect("storage bucket in environment"),
            working_bucket: std::env::var("WORKING_BUCKET").expect("working bucket in environment"),
            labels_table: std::env::var("LABELS_TABLE").expect("labels table in environment"),
            notification_arn: std::env::var("").expect("notification channel in environment"),
        }
    }

    pub fn sdk_config(&self) -> &SdkConfig {
        &self.sdk_config
    }

    pub fn dynamodb_client(&self) -> &aws_sdk_dynamodb::Client {
        &self.dynamodb_client
    }

    pub fn rekognition_client(&self) -> &aws_sdk_rekognition::Client {
        &self.rekognition_client
    }

    pub fn s3_client(&self) -> &aws_sdk_s3::Client {
        &self.s3_client
    }

    pub fn sns_client(&self) -> &aws_sdk_sns::Client {
        &self.sns_client
    }

    pub fn storage_bucket(&self) -> &String {
        &self.storage_bucket
    }

    pub fn working_bucket(&self) -> &String {
        &self.working_bucket
    }

    pub fn labels_table(&self) -> &String {
        &self.labels_table
    }

    pub fn notification_arn(&self) -> &String {
        &self.notification_arn
    }
}

#[macro_export]
macro_rules! apig_response(
  ($body:expr) => {{
    let headers = http::header::HeaderMap::new();
    aws_lambda_events::apigw::ApiGatewayProxyResponse {
      status_code: 200,
      headers,
      multi_value_headers: http::header::HeaderMap::new(),
      body: Some(aws_lambda_events::encodings::Body::Text(serde_json::json!($body).to_string())),
      is_base64_encoded: Some(true)
    }
  }}
);
