#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderDetailPageDTO {
    pub order_id: Option<String>,
    pub dish_id: Option<String>,
    pub setmeal_id: Option<String>,
}
