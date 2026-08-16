use crate::domain::dto::category_dto::CategoryPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

// Make Cargo track the mapper file so HTML-only edits re-run `html_sql!` code generation.
const _: &str = include_str!("category.html");

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub id: Option<String>,
    pub category_type: Option<i32>,
    pub name: Option<String>,
    pub sort: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_user: Option<String>,
    pub update_user: Option<String>,
}

crud!(Category {});

#[html_sql("src/domain/table/category.html")]
impl Category {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &CategoryPageDTO) -> rbatis::Result<rbatis::Page<Category>> { impled!() }
}
