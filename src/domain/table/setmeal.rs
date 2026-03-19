use crate::domain::dto::setmeal_dto::SetmealPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};
use rbatis::executor::Executor;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Setmeal {
    pub id: Option<String>,
    pub category_id: Option<String>,
    pub name: Option<String>,
    pub price: Option<String>,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetmealWithDish {
    pub id: Option<String>,
    pub category_id: Option<String>,
    pub name: Option<String>,
    pub price: Option<String>,
    pub status: Option<i32>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_user: Option<String>,
    pub update_user: Option<String>,
    pub is_deleted: Option<i32>,
    pub setmeal_dish_id: Option<String>,
    pub dish_id: Option<String>,
    pub dish_name: Option<String>,
    pub dish_price: Option<String>,
    pub copies: Option<i32>,
    pub sort: Option<i32>,
}

crud!(Setmeal {});

#[html_sql("src/domain/table/setmeal.html")]
impl Setmeal {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &SetmealPageDTO) -> rbatis::Result<rbatis::Page<Setmeal>> { impled!() }
    pub async fn select_setmeal_with_dishes(conn: &dyn Executor, setmeal_id: Option<&str>, name: Option<&str>) -> rbatis::Result<Vec<SetmealWithDish>> { impled!() }
}
