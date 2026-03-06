#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EmployeePageDTO {
    pub name: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i32>,
}
