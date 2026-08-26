use crate::domain::dto::dish_flavor_dto::DishFlavorPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DishFlavor {
    pub id: Option<String>,
    pub dish_id: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_user: Option<String>,
    pub update_user: Option<String>,
    pub is_deleted: Option<i32>,
}

crud!(DishFlavor {});

#[html_sql("src/domain/table/dish_flavor.html")]
impl DishFlavor {
    pub async fn select_page(
        conn: &dyn rbatis::Executor,
        page_request: &rbatis::PageRequest,
        dto: &DishFlavorPageDTO,
    ) -> rbatis::Result<rbatis::Page<DishFlavor>> {
        impled!()
    }
}
