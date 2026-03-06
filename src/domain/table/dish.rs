use crate::domain::dto::dish_dto::DishPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Dish {
    pub id: Option<String>,
    pub name: Option<String>,
    pub category_id: Option<String>,
    pub price: Option<f64>,
    pub code: Option<String>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub status: Option<i32>,
    pub sort: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_user: Option<String>,
    pub update_user: Option<String>,
    pub is_deleted: Option<i32>,
}

crud!(Dish {});

#[html_sql("src/domain/table/dish.html")]
impl Dish {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &DishPageDTO) -> rbatis::Result<rbatis::Page<Dish>> { impled!() }
}
