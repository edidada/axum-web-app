use crate::domain::dto::setmeal_dish_dto::SetmealDishPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetmealDish {
    pub id: Option<String>,
    pub setmeal_id: Option<String>,
    pub dish_id: Option<String>,
    pub name: Option<String>,
    pub price: Option<String>,
    pub copies: Option<i32>,
    pub sort: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_user: Option<String>,
    pub update_user: Option<String>,
    pub is_deleted: Option<i32>,
}

crud!(SetmealDish {});

#[html_sql("src/domain/table/setmeal_dish.html")]
impl SetmealDish {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &SetmealDishPageDTO) -> rbatis::Result<rbatis::Page<SetmealDish>> { impled!() }
}
