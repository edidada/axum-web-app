#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AddressBookPageDTO {
    pub user_id: Option<String>,
    pub consignee: Option<String>,
    pub phone: Option<String>,
    pub label: Option<String>,
    pub is_default: Option<i32>,
}
