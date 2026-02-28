use axum::{response::Json, extract::Extension, http::StatusCode};
use serde::{Deserialize, Serialize};
use rbatis::rbatis::RBatis;
use rbs::Value;
use std::error::Error;

#[derive(Deserialize)]
pub struct PaginateRequest {
    sql: String,
    page: u32,
    page_size: u32,
}

#[derive(Serialize)]
pub struct PaginateResponse {
    total: u64,
    page: u32,
    page_size: u32,
    data: Vec<Value>,
}

pub async fn paginate_handler(
    Json(payload): Json<PaginateRequest>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<PaginateResponse>, (StatusCode, String)> {
    let result = paginate_query(rb, &payload.sql, payload.page, payload.page_size).await;
    match result {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn paginate_query(
    rb: &RBatis,
    sql: &str,
    page: u32,
    page_size: u32,
) -> Result<PaginateResponse, Box<dyn Error>> {
    // 计算偏移量
    let offset = (page - 1) * page_size;
    
    // 构建带分页的 SQL
    let paginated_sql = format!("{} LIMIT {} OFFSET {}", sql, page_size, offset);
    
    // 执行查询获取数据
    let data: Vec<Value> = rb.query_decode(&paginated_sql, vec![]).await?;
    
    // 构建获取总数的 SQL
    let count_sql = format!("SELECT COUNT(*) FROM ({}) as total", sql);
    
    // 执行查询获取总数
    let count_result: Vec<Value> = rb.query_decode(&count_sql, vec![]).await?;
    let mut total = 0;
    if let Some(Value::Map(map)) = count_result.first() {
        // 遍历键值对查找 COUNT(*)
        for (key, value) in map {
            if let Value::String(key_str) = key {
                if key_str == "COUNT(*)" {
                    match value {
                        Value::I32(count) => total = *count as u64,
                        Value::I64(count) => total = *count as u64,
                        Value::U32(count) => total = *count as u64,
                        Value::U64(count) => total = *count,
                        _ => (),
                    }
                    break;
                }
            }
        }
    };
    
    Ok(PaginateResponse {
        total,
        page,
        page_size,
        data,
    })
}
