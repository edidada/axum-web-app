#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DishFlavorPageDTO {
    pub dish_id: Option<String>,
    pub name: Option<String>,
}
