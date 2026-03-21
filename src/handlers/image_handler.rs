use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::infrastructure::minio_client::get_minio_client;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadImageResponse {
    pub success: bool,
    pub message: String,
    pub url: Option<String>,
    pub object_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ImageQuery {
    pub url: String,
}

/// 从 URL 下载图片并上传到 MinIO
pub async fn upload_image_from_url(
    Query(query): Query<ImageQuery>,
) -> impl IntoResponse {
    let client = match get_minio_client() {
        Some(c) => c,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UploadImageResponse {
                    success: false,
                    message: "MinIO client not initialized".to_string(),
                    url: None,
                    object_name: None,
                }),
            );
        }
    };

    // 生成对象名称
    let object_name = format!("images/{}.jpg", uuid::Uuid::new_v4());

    // 下载并上传图片
    match client.download_and_upload_image(&query.url, &object_name).await {
        Ok(url) => {
            tracing::info!("Image uploaded successfully: {}", url);
            (
                StatusCode::OK,
                Json(UploadImageResponse {
                    success: true,
                    message: "Image uploaded successfully".to_string(),
                    url: Some(url),
                    object_name: Some(object_name),
                }),
            )
        }
        Err(e) => {
            tracing::error!("Failed to upload image: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UploadImageResponse {
                    success: false,
                    message: format!("Failed to upload image: {}", e),
                    url: None,
                    object_name: None,
                }),
            )
        }
    }
}

/// 删除 MinIO 中的图片
pub async fn delete_image(
    Path(object_name): Path<String>,
) -> impl IntoResponse {
    let client = match get_minio_client() {
        Some(c) => c,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "MinIO client not initialized"
                })),
            );
        }
    };

    match client.delete_file(&object_name).await {
        Ok(_) => {
            tracing::info!("Image deleted successfully: {}", object_name);
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "success": true,
                    "message": "Image deleted successfully"
                })),
            )
        }
        Err(e) => {
            tracing::error!("Failed to delete image: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": format!("Failed to delete image: {}", e)
                })),
            )
        }
    }
}

/// 获取图片的 MinIO URL
pub async fn get_image_url(
    Path(object_name): Path<String>,
) -> impl IntoResponse {
    let client = match get_minio_client() {
        Some(c) => c,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "MinIO client not initialized"
                })),
            );
        }
    };

    let url = client.get_file_url(&object_name);
    
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "url": url
        })),
    )
}
