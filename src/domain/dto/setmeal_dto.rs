#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SetmealPageDTO {
    pub name: Option<String>,
    pub category_id: Option<String>,
    pub status: Option<i32>,
}
