use axum::Json;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: u64,
    pub version: String,
    pub uptime_seconds: u64,
}

#[derive(Serialize)]
pub struct SystemInfo {
    pub database_connected: bool,
    pub memory_usage_mb: u64,
    pub active_connections: u32,
}

#[derive(Serialize)]
pub struct DetailedHealthResponse {
    pub status: String,
    pub timestamp: u64,
    pub version: String,
    pub uptime_seconds: u64,
    pub system: SystemInfo,
}

// 应用启动时间
static START_TIME: std::sync::OnceLock<SystemTime> = std::sync::OnceLock::new();

pub fn init_start_time() {
    START_TIME.get_or_init(SystemTime::now);
}

/// 健康检查接口 - 基础健康状态
pub async fn health_check() -> Json<HealthResponse> {
    let current_time = SystemTime::now();
    let start_time = START_TIME.get().unwrap_or(&current_time);
    let uptime = SystemTime::now()
        .duration_since(*start_time)
        .unwrap_or_default()
        .as_secs();

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    tracing::info!("Health check requested, uptime: {}s", uptime);

    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp,
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
    })
}

/// 详细健康检查接口 - 包含系统信息
pub async fn health_check_detailed() -> Json<DetailedHealthResponse> {
    let current_time = SystemTime::now();
    let start_time = START_TIME.get().unwrap_or(&current_time);
    let uptime = SystemTime::now()
        .duration_since(*start_time)
        .unwrap_or_default()
        .as_secs();

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // 检查数据库连接
    let db_connected = check_database_connection().await;

    tracing::info!(
        "Detailed health check requested, uptime: {}s, db_connected: {}",
        uptime,
        db_connected
    );

    Json(DetailedHealthResponse {
        status: if db_connected { "healthy" } else { "degraded" }.to_string(),
        timestamp,
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        system: SystemInfo {
            database_connected: db_connected,
            memory_usage_mb: get_memory_usage(),
            // This application does not maintain a connection-pool metric yet.
            active_connections: 0,
        },
    })
}

/// 就绪检查接口 - 用于 Kubernetes 等环境
pub async fn readiness_check() -> Json<serde_json::Value> {
    let ready = check_database_connection().await;

    Json(serde_json::json!({
        "ready": ready,
        "timestamp": SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }))
}

/// 存活检查接口 - 用于 Kubernetes 等环境
pub async fn liveness_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "alive": true,
        "timestamp": SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }))
}

// 检查数据库连接
async fn check_database_connection() -> bool {
    // 简化实现，实际应该尝试执行一个简单的查询
    true
}

// 获取内存使用量（简化实现）
fn get_memory_usage() -> u64 {
    // 在实际应用中，可以使用 sysinfo 等库获取真实内存使用
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn detailed_health_check_includes_system_info() {
        init_start_time();
        let response = health_check_detailed().await.0;

        assert!(response.system.database_connected);
        assert_eq!(response.system.active_connections, 0);
        assert!(!response.version.is_empty());
    }
}
