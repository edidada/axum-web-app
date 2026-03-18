#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct OrdersPageDTO {
    pub number: Option<String>,
    pub status: Option<i32>,
    pub user_id: Option<String>,
    pub address_book_id: Option<String>,
}
