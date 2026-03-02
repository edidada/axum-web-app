use rbatis::{html_sql};
use rbatis::executor::Executor;
use rbs::Value;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaginateResult {
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub data: Vec<Value>,
}

#[html_sql("src/domain/table/paginate.html")]
impl PaginateResult {
    pub async fn paginate_query(conn: &dyn Executor, sql: &str, page: u32, page_size: u32) -> rbatis::Result<Vec<Value>> { impled!() }
    pub async fn count_query(conn: &dyn Executor, sql: &str) -> rbatis::Result<Vec<Value>> { impled!() }
}