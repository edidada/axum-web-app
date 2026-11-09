use rbatis::executor::Executor;
use rbatis::html_sql;
use rbs::Value;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaginateResult {
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub data: Vec<Value>,
}

/// Typed entry points for the statements in `paginate.html`.
/// `query` is intentionally a raw fragment because this mapper is for internally
/// constructed pagination queries; do not pass untrusted SQL into it.
pub struct PaginateMapper;

#[html_sql("src/domain/table/paginate.html")]
impl PaginateMapper {
    pub async fn paginate_query(
        conn: &dyn Executor,
        query: &str,
        page_size: u32,
        offset: u32,
    ) -> rbatis::Result<Vec<Value>> {
        impled!()
    }

    pub async fn count_query(conn: &dyn Executor, query: &str) -> rbatis::Result<i64> {
        impled!()
    }
}
