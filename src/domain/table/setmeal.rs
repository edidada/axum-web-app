use crate::domain::dto::setmeal_dto::SetmealPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Setmeal {
    pub id: Option<String>,
    pub category_id: Option<String>,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub status: Option<i32>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_user: Option<String>,
    pub update_user: Option<String>,
    pub is_deleted: Option<i32>,
}

crud!(Setmeal {});

#[html_sql("src/domain/table/setmeal.html")]
impl Setmeal {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &SetmealPageDTO) -> rbatis::Result<rbatis::Page<Setmeal>> { impled!() }
}
