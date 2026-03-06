#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DishPageDTO {
    pub name: Option<String>,
    pub category_id: Option<String>,
    pub status: Option<i32>,
}
