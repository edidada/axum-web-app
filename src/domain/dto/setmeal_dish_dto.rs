#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetmealDishPageDTO {
    pub setmeal_id: Option<String>,
    pub dish_id: Option<String>,
}
