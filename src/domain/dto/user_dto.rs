#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UserPageDTO {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i32>,
}
