use crate::domain::dto::shopping_cart_dto::ShoppingCartPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ShoppingCart {
    pub id: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub user_id: Option<String>,
    pub dish_id: Option<String>,
    pub setmeal_id: Option<String>,
    pub dish_flavor: Option<String>,
    pub number: Option<i32>,
    pub amount: Option<String>,
    pub create_time: Option<DateTime>,
}

crud!(ShoppingCart {});

#[html_sql("/Users/ibqo/Develop/git/github/rust/axum-web-app/src/domain/table/shopping_cart.html")]
impl ShoppingCart {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &ShoppingCartPageDTO) -> rbatis::Result<rbatis::Page<ShoppingCart>> { impled!() }
}
