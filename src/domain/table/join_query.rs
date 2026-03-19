use crate::domain::dto::join_dto::*;
use rbatis::executor::Executor;
use rbatis::html_sql;

// JOIN 查询结构体
#[derive(Clone, Debug)]
pub struct JoinQuery;

#[html_sql("src/domain/table/join_queries.html")]
impl JoinQuery {
    /// 查询订单及其详情
    pub async fn select_order_with_details(
        conn: &dyn Executor,
        order_id: Option<&str>,
        user_id: Option<&str>,
        status: Option<i32>,
    ) -> rbatis::Result<Vec<OrderWithDetailDTO>> {
        impled!()
    }

    /// 查询套餐及其包含的菜品
    pub async fn select_setmeal_with_dishes(
        conn: &dyn Executor,
        setmeal_id: Option<&str>,
        category_id: Option<&str>,
        status: Option<i32>,
    ) -> rbatis::Result<Vec<SetmealWithDishDTO>> {
        impled!()
    }

    /// 查询购物车及商品信息
    pub async fn select_shopping_cart_with_items(
        conn: &dyn Executor,
        user_id: Option<&str>,
    ) -> rbatis::Result<Vec<ShoppingCartWithItemDTO>> {
        impled!()
    }

    /// 查询菜品完整信息（含分类和口味）
    pub async fn select_dish_full_info(
        conn: &dyn Executor,
        dish_id: Option<&str>,
        category_id: Option<&str>,
        status: Option<i32>,
    ) -> rbatis::Result<Vec<DishFullInfoDTO>> {
        impled!()
    }

    /// 查询用户及订单统计
    pub async fn select_user_with_order_stats(
        conn: &dyn Executor,
        user_id: Option<&str>,
        phone: Option<&str>,
    ) -> rbatis::Result<Vec<UserWithOrderStatsDTO>> {
        impled!()
    }

    /// 查询地址簿及用户信息
    pub async fn select_address_book_with_user(
        conn: &dyn Executor,
        user_id: Option<&str>,
        is_default: Option<i32>,
    ) -> rbatis::Result<Vec<AddressBookWithUserDTO>> {
        impled!()
    }
}

// 地址簿 JOIN 用户 DTO
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AddressBookWithUserDTO {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub user_phone: Option<String>,
    pub consignee: Option<String>,
    pub sex: Option<i32>,
    pub phone: Option<String>,
    pub province_code: Option<String>,
    pub province_name: Option<String>,
    pub city_code: Option<String>,
    pub city_name: Option<String>,
    pub district_code: Option<String>,
    pub district_name: Option<String>,
    pub detail: Option<String>,
    pub label: Option<String>,
    pub is_default: Option<i32>,
}
