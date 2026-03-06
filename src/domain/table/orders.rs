use crate::domain::dto::orders_dto::OrdersPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Orders {
    pub id: Option<String>,
    pub number: Option<String>,
    pub status: Option<i32>,
    pub user_id: Option<String>,
    pub address_book_id: Option<String>,
    pub order_time: Option<DateTime>,
    pub checkout_time: Option<DateTime>,
    pub pay_method: Option<i32>,
    pub amount: Option<f64>,
    pub remark: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub user_name: Option<String>,
    pub consignee: Option<String>,
}

crud!(Orders {});

#[html_sql("src/domain/table/orders.html")]
impl Orders {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &OrdersPageDTO) -> rbatis::Result<rbatis::Page<Orders>> { impled!() }
}
