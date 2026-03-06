use crate::domain::dto::employee_dto::EmployeePageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Employee {
    pub id: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<String>,
    pub id_number: Option<String>,
    pub status: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_user: Option<String>,
    pub update_user: Option<String>,
}

crud!(Employee {});

#[html_sql("src/domain/table/employee.html")]
impl Employee {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &EmployeePageDTO) -> rbatis::Result<rbatis::Page<Employee>> { impled!() }
}
