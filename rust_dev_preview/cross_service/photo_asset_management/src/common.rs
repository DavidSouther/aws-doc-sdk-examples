use aws_config::SdkConfig;

pub fn init_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_ansi(false)
        .without_time()
        .init();
}

// Common fields are loaded during the Lambda init phase. These include reading
// several environment variables to know which buckets and tables to work from,
// as well as preparing the SDK Config (expensive) and several clients from that
// config (cheap).
pub struct Common {
    sdk_config: SdkConfig,
    rekognition_client: aws_sdk_rekognition::Client,
    dynamodb_client: aws_sdk_dynamodb::Client,
    s3_client: aws_sdk_s3::Client,
    sns_client: aws_sdk_sns::Client,
    storage_bucket: String,
    working_bucket: String,
    labels_table: String,
    notification_topic: String,
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
            // PAM environment is declared in the cdk, in lib/backend/lambdas.ts
            storage_bucket: std::env::var("STORAGE_BUCKET_NAME")
                .expect("storage bucket in environment"),
            working_bucket: std::env::var("WORKING_BUCKET_NAME")
                .expect("working bucket in environment"),
            labels_table: std::env::var("LABELS_TABLE_NAME").expect("labels table in environment"),
            notification_topic: std::env::var("NOTIFICATION_TOPIC")
                .expect("notification topic in environment"),
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

    pub fn notification_topic(&self) -> &String {
        &self.notification_topic
    }
}

#[macro_export]
macro_rules! apig_response(
  ($body:expr) => {{
    let mut headers = http::header::HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", http::header::HeaderValue::from_static("*"));
    aws_lambda_events::apigw::ApiGatewayProxyResponse {
      status_code: 200,
      headers,
      multi_value_headers: http::header::HeaderMap::new(),
      body: Some(aws_lambda_events::encodings::Body::Text(serde_json::json!($body).to_string())),
      is_base64_encoded: None
    }
  }}
);
