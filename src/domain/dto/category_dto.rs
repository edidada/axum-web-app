#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CategoryPageDTO {
    pub r#type: Option<i32>,
    pub name: Option<String>,
}
