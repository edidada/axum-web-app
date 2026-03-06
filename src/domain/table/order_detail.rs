use crate::domain::dto::order_detail_dto::OrderDetailPageDTO;

use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderDetail {
    pub id: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub order_id: Option<String>,
    pub dish_id: Option<String>,
    pub setmeal_id: Option<String>,
    pub dish_flavor: Option<String>,
    pub number: Option<i32>,
    pub amount: Option<f64>,
}

crud!(OrderDetail {});

#[html_sql("src/domain/table/order_detail.html")]
impl OrderDetail {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &OrderDetailPageDTO) -> rbatis::Result<rbatis::Page<OrderDetail>> { impled!() }
}
