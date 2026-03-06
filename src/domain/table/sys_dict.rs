use crate::domain::dto::dict_dto::DictPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::executor::Executor;
use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

crud!(SysDict {});

#[html_sql("src/domain/table/sys_dict.html")]
impl SysDict {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &DictPageDTO) -> rbatis::Result<rbatis::Page<SysDict>> { impled!() }

    // pub async fn insert(&self, conn: &dyn Executor) -> rbatis::Result<usize> { impled!() }
    //
    // pub async fn update_by_id(&self, conn: &dyn Executor) -> rbatis::Result<usize> { impled!() }
    //
    // pub async fn delete_by_id(conn: &dyn Executor, id: &str) -> rbatis::Result<usize> { impled!() }
    //
    // pub async fn select_by_id(conn: &dyn Executor, id: &str) -> rbatis::Result<Option<SysDict>> { impled!() }
}