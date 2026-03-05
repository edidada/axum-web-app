use rbs::Value;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaginateResult {
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub data: Vec<Value>,
}
