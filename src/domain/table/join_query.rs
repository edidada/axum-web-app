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

    /// 查询购物车及其关联的菜品/套餐信息
    pub async fn select_shopping_cart_with_items(
        conn: &dyn Executor,
        user_id: Option<&str>,
        cart_id: Option<&str>,
    ) -> rbatis::Result<Vec<CartWithItemDTO>> {
        impled!()
    }

    /// 查询菜品完整信息
    pub async fn select_dish_full_info(
        conn: &dyn Executor,
        dish_id: Option<&str>,
        category_id: Option<&str>,
        status: Option<i32>,
    ) -> rbatis::Result<Vec<DishFullInfoDTO>> {
        impled!()
    }

    /// 查询用户及其订单统计
    pub async fn select_user_with_order_stats(
        conn: &dyn Executor,
        user_id: Option<&str>,
        phone: Option<&str>,
    ) -> rbatis::Result<Vec<UserWithOrderStatsDTO>> {
        impled!()
    }

    /// 查询地址簿及其关联的用户信息
    pub async fn select_address_book_with_user(
        conn: &dyn Executor,
        user_id: Option<&str>,
        address_id: Option<&str>,
        is_default: Option<i32>,
    ) -> rbatis::Result<Vec<AddressBookWithUserDTO>> {
        impled!()
    }
}
