use aws_config::meta::region::RegionProviderChain;
use aws_credential_types::Credentials;
use aws_sdk_s3::{Client, Config};
use aws_sdk_s3::config::{BehaviorVersion, Region};
use aws_sdk_s3::primitives::ByteStream;
use std::time::Duration;

/// MinIO 配置
#[derive(Clone, Debug)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub region: String,
}

impl Default for MinioConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:9000".to_string(),
            access_key: "minioadmin".to_string(),
            secret_key: "minioadmin".to_string(),
            bucket: "images".to_string(),
            region: "us-east-1".to_string(),
        }
    }
}

/// MinIO 客户端
#[derive(Clone)]
pub struct MinioClient {
    client: Client,
    config: MinioConfig,
}

impl MinioClient {
    /// 创建新的 MinIO 客户端
    pub async fn new(config: MinioConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let credentials = Credentials::new(
            &config.access_key,
            &config.secret_key,
            None,
            None,
            "minio",
        );

        let sdk_config = Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(config.region.clone()))
            .credentials_provider(credentials)
            .endpoint_url(&config.endpoint)
            .force_path_style(true) // MinIO 需要这个
            .build();

        let client = Client::from_conf(sdk_config);

        Ok(Self { client, config })
    }

    /// 使用默认配置创建客户端
    pub async fn default() -> Result<Self, Box<dyn std::error::Error>> {
        Self::new(MinioConfig::default()).await
    }

    /// 创建存储桶（如果不存在）
    pub async fn create_bucket_if_not_exists(&self) -> Result<(), Box<dyn std::error::Error>> {
        let bucket_name = &self.config.bucket;
        
        // 尝试创建桶，如果已存在会返回错误
        match self.client
            .create_bucket()
            .bucket(bucket_name)
            .send()
            .await {
            Ok(_) => {
                tracing::info!("Created bucket: {}", bucket_name);
            }
            Err(e) => {
                // 如果桶已存在，忽略错误
                let err_str = format!("{:?}", e);
                if err_str.contains("BucketAlreadyExists") || err_str.contains("AlreadyOwnedByYou") {
                    tracing::debug!("Bucket already exists: {}", bucket_name);
                } else {
                    return Err(Box::new(e));
                }
            }
        }

        Ok(())
    }

    /// 上传文件
    pub async fn upload_file(
        &self,
        object_name: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let stream = ByteStream::from(data);
        
        self.client
            .put_object()
            .bucket(&self.config.bucket)
            .key(object_name)
            .body(stream)
            .content_type(content_type)
            .send()
            .await?;

        // 返回文件的访问 URL
        let url = format!("{}/{}/{}", self.config.endpoint, self.config.bucket, object_name);
        Ok(url)
    }

    /// 下载文件
    pub async fn download_file(&self, object_name: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let response = self.client
            .get_object()
            .bucket(&self.config.bucket)
            .key(object_name)
            .send()
            .await?;

        let data = response.body.collect().await?.into_bytes().to_vec();
        Ok(data)
    }

    /// 删除文件
    pub async fn delete_file(&self, object_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .delete_object()
            .bucket(&self.config.bucket)
            .key(object_name)
            .send()
            .await?;

        Ok(())
    }

    /// 生成文件的访问 URL
    pub fn get_file_url(&self, object_name: &str) -> String {
        format!("{}/{}/{}", self.config.endpoint, self.config.bucket, object_name)
    }

    /// 从 URL 下载图片并上传到 MinIO
    pub async fn download_and_upload_image(
        &self,
        image_url: &str,
        object_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 下载图片
        let response = reqwest::get(image_url).await?;
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("image/jpeg")
            .to_string();
        
        let image_data = response.bytes().await?.to_vec();

        // 上传到 MinIO
        let url = self.upload_file(object_name, image_data, &content_type).await?;
        
        tracing::info!("Downloaded and uploaded image: {} -> {}", image_url, url);
        
        Ok(url)
    }
}

/// 全局 MinIO 客户端实例
use once_cell::sync::OnceCell;
static MINIO_CLIENT: OnceCell<MinioClient> = OnceCell::new();

/// 初始化全局 MinIO 客户端
pub async fn init_minio_client() -> Result<(), Box<dyn std::error::Error>> {
    let client = MinioClient::default().await?;
    client.create_bucket_if_not_exists().await?;
    
    MINIO_CLIENT.set(client).map_err(|_| "MinIO client already initialized")?;
    
    tracing::info!("MinIO client initialized successfully");
    Ok(())
}

/// 获取全局 MinIO 客户端
pub fn get_minio_client() -> Option<&'static MinioClient> {
    MINIO_CLIENT.get()
}
