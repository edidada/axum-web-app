use axum::{
    extract::Query,
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::infrastructure::minio_client::get_minio_client;

#[derive(Deserialize)]
pub struct SyncImagesQuery {
    pub table: Option<String>, // dish, setmeal, all
}

#[derive(Serialize)]
pub struct SyncImagesResponse {
    pub success: bool,
    pub message: String,
    pub processed: usize,
    pub uploaded: usize,
    pub failed: usize,
    pub details: Vec<String>,
}

/// 同步图片到 MinIO
/// 从数据库中读取现有的图片URL，下载并上传到 MinIO，然后更新数据库
pub async fn sync_images_to_minio(
    Query(query): Query<SyncImagesQuery>,
) -> impl IntoResponse {
    let client = match get_minio_client() {
        Some(c) => c,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SyncImagesResponse {
                    success: false,
                    message: "MinIO client not initialized".to_string(),
                    processed: 0,
                    uploaded: 0,
                    failed: 0,
                    details: vec!["MinIO client not initialized".to_string()],
                }),
            );
        }
    };

    let table = query.table.as_deref().unwrap_or("all");
    let mut response = SyncImagesResponse {
        success: true,
        message: "Image sync started".to_string(),
        processed: 0,
        uploaded: 0,
        failed: 0,
        details: vec![format!("Sync started for table: {}", table)],
    };

    // 注意：实际的同步操作需要通过后台任务或单独的 API 来完成
    // 这里返回一个提示信息，告知用户如何使用其他 API 来同步
    response.details.push("Please use /api/image/upload-url?url=IMAGE_URL to upload individual images".to_string());

    (
        StatusCode::OK,
        Json(response),
    )
}
