use axum::{
    extract::Query,
    Extension, Json,
};
use serde::Deserialize;
use rbatis::RBatis;

use crate::domain::table::join_query::JoinQuery;

// 通用响应结构
#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            code: 500,
            message: message.to_string(),
            data: None,
        }
    }
}

// 订单详情查询参数
#[derive(Deserialize)]
pub struct OrderWithDetailsQuery {
    pub order_id: Option<String>,
    pub user_id: Option<String>,
    pub status: Option<i32>,
}

// 套餐菜品查询参数
#[derive(Deserialize)]
pub struct SetmealWithDishesQuery {
    pub setmeal_id: Option<String>,
    pub category_id: Option<String>,
    pub status: Option<i32>,
}

// 购物车查询参数
#[derive(Deserialize)]
pub struct ShoppingCartQuery {
    pub user_id: Option<String>,
}

// 菜品完整信息查询参数
#[derive(Deserialize)]
pub struct DishFullInfoQuery {
    pub dish_id: Option<String>,
    pub category_id: Option<String>,
    pub status: Option<i32>,
}

// 用户订单统计查询参数
#[derive(Deserialize)]
pub struct UserOrderStatsQuery {
    pub user_id: Option<String>,
    pub phone: Option<String>,
}

// 地址簿查询参数
#[derive(Deserialize)]
pub struct AddressBookWithUserQuery {
    pub user_id: Option<String>,
    pub is_default: Option<i32>,
}

/// 查询订单及其详情
pub async fn get_order_with_details(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<OrderWithDetailsQuery>,
) -> Json<ApiResponse<Vec<crate::domain::dto::join_dto::OrderWithDetailDTO>>> {
    match JoinQuery::select_order_with_details(
        rb,
        params.order_id.as_deref(),
        params.user_id.as_deref(),
        params.status,
    )
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => {
            tracing::error!("查询订单详情失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 查询套餐及其菜品
pub async fn get_setmeal_with_dishes(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<SetmealWithDishesQuery>,
) -> Json<ApiResponse<Vec<crate::domain::dto::join_dto::SetmealWithDishDTO>>> {
    match JoinQuery::select_setmeal_with_dishes(
        rb,
        params.setmeal_id.as_deref(),
        params.category_id.as_deref(),
        params.status,
    )
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => {
            tracing::error!("查询套餐菜品失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 查询购物车及商品信息
pub async fn get_shopping_cart_with_items(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<ShoppingCartQuery>,
) -> Json<ApiResponse<Vec<crate::domain::dto::join_dto::ShoppingCartWithItemDTO>>> {
    match JoinQuery::select_shopping_cart_with_items(
        rb,
        params.user_id.as_deref(),
    )
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => {
            tracing::error!("查询购物车失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 查询菜品完整信息
pub async fn get_dish_full_info(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<DishFullInfoQuery>,
) -> Json<ApiResponse<Vec<crate::domain::dto::join_dto::DishFullInfoDTO>>> {
    match JoinQuery::select_dish_full_info(
        rb,
        params.dish_id.as_deref(),
        params.category_id.as_deref(),
        params.status,
    )
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => {
            tracing::error!("查询菜品信息失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 查询用户及订单统计
pub async fn get_user_with_order_stats(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<UserOrderStatsQuery>,
) -> Json<ApiResponse<Vec<crate::domain::dto::join_dto::UserWithOrderStatsDTO>>> {
    match JoinQuery::select_user_with_order_stats(
        rb,
        params.user_id.as_deref(),
        params.phone.as_deref(),
    )
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => {
            tracing::error!("查询用户统计失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 查询地址簿及用户信息
pub async fn get_address_book_with_user(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<AddressBookWithUserQuery>,
) -> Json<ApiResponse<Vec<crate::domain::table::join_query::AddressBookWithUserDTO>>> {
    match JoinQuery::select_address_book_with_user(
        rb,
        params.user_id.as_deref(),
        params.is_default,
    )
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => {
            tracing::error!("查询地址簿失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}
