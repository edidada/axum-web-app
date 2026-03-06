#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CategoryPageDTO {
    pub category_type: Option<i32>,
    pub name: Option<String>,
}
