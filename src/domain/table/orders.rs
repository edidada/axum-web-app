use crate::domain::dto::orders_dto::OrdersPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};
use rbatis::executor::Executor;

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
    pub pay_status: Option<i32>,
    pub amount: Option<String>,
    pub remark: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub user_name: Option<String>,
    pub consignee: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderWithDetail {
    pub id: Option<String>,
    pub number: Option<String>,
    pub status: Option<i32>,
    pub user_id: Option<String>,
    pub address_book_id: Option<String>,
    pub order_time: Option<DateTime>,
    pub checkout_time: Option<DateTime>,
    pub pay_method: Option<i32>,
    pub amount: Option<String>,
    pub remark: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub user_name: Option<String>,
    pub consignee: Option<String>,
    pub detail_id: Option<String>,
    pub detail_name: Option<String>,
    pub detail_image: Option<String>,
    pub dish_id: Option<String>,
    pub setmeal_id: Option<String>,
    pub dish_flavor: Option<String>,
    pub detail_number: Option<i32>,
    pub detail_amount: Option<String>,
}

crud!(Orders {});

#[html_sql("src/domain/table/orders.html")]
impl Orders {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &OrdersPageDTO) -> rbatis::Result<rbatis::Page<Orders>> { impled!() }
    pub async fn select_order_with_details(conn: &dyn Executor, order_id: Option<&str>, user_id: Option<&str>) -> rbatis::Result<Vec<OrderWithDetail>> { impled!() }
}
