use crate::domain::table::paginate::PaginateResult;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::Json;
use rbatis::rbatis::RBatis;
use rbs::Value;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct PaginateRequest {
    sql: String,
    page: u32,
    page_size: u32,
}

/// HTTP response reuses the typed pagination result shared by mapper callers.
pub type PaginateResponse = PaginateResult;

pub async fn paginate_query(
    rb: &RBatis,
    sql: &str,
    page: u32,
    page_size: u32,
) -> Result<PaginateResponse, Box<dyn Error>> {
    paginate_query_with_rbs_params(rb, sql, page, page_size, vec![]).await
}

async fn paginate_query_with_rbs_params(
    rb: &RBatis,
    sql: &str,
    page: u32,
    page_size: u32,
    params: Vec<Value>,
) -> Result<PaginateResponse, Box<dyn Error>> {
    if page == 0 || page_size == 0 {
        return Err("page and page_size must be greater than zero".into());
    }

    // 计算偏移量
    let offset = (page - 1) * page_size;

    // 构建带分页的 SQL
    let paginated_sql = format!("{} LIMIT {} OFFSET {}", sql, page_size, offset);

    // 执行查询获取数据
    let data: Vec<Value> = rb.query_decode(&paginated_sql, params.clone()).await?;

    // 构建获取总数的 SQL
    let count_sql = format!("SELECT COUNT(*) FROM ({}) as total", sql);

    // 执行查询获取总数
    let count_result: Vec<Value> = rb.query_decode(&count_sql, params).await?;
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

#[derive(Deserialize)]
pub struct PaginateWithParamsRequest {
    sql: String,
    page: u32,
    page_size: u32,
    params: Vec<serde_json::Value>,
}

pub async fn paginate_query_with_params(
    rb: &RBatis,
    sql: &str,
    page: u32,
    page_size: u32,
    params: Vec<serde_json::Value>,
) -> Result<PaginateResponse, Box<dyn Error>> {
    let params = params.into_iter().map(|value| rbs::value!(value)).collect();
    paginate_query_with_rbs_params(rb, sql, page, page_size, params).await
}

pub async fn paginate_with_params_handler(
    Json(payload): Json<PaginateWithParamsRequest>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<PaginateResponse>, (StatusCode, String)> {
    let result = paginate_query_with_params(
        rb,
        &payload.sql,
        payload.page,
        payload.page_size,
        payload.params,
    )
    .await;
    match result {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paginate_response_keeps_page_metadata_and_rows() {
        let response = PaginateResponse {
            total: 7,
            page: 2,
            page_size: 3,
            data: vec![rbs::value!({"id": "order-4"})],
        };

        assert_eq!(response.total, 7, "total row count should be preserved");
        assert_eq!(response.page, 2, "requested page should be preserved");
        assert_eq!(response.page_size, 3, "page size should be preserved");
        assert_eq!(response.data.len(), 1, "the fixture should contain one row");
    }
}
