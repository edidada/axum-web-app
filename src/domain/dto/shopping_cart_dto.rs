#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ShoppingCartPageDTO {
    pub user_id: Option<String>,
    pub dish_id: Option<String>,
    pub setmeal_id: Option<String>,
}
