//! 报表查询 Handler
//! 
//! 提供各种业务报表和统计查询接口，包括：
//! - 订单详情报表
//! - 套餐菜品关联报表
//! - 购物车商品报表
//! - 菜品完整信息报表
//! - 用户订单统计报表
//! - 地址簿用户信息报表

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

// ==================== 查询参数定义 ====================

/// 订单详情查询参数
#[derive(Deserialize)]
pub struct OrderDetailsReportQuery {
    pub order_id: Option<String>,
    pub user_id: Option<String>,
    pub status: Option<i32>,
}

/// 套餐菜品查询参数
#[derive(Deserialize)]
pub struct SetmealDishesReportQuery {
    pub setmeal_id: Option<String>,
    pub category_id: Option<String>,
    pub status: Option<i32>,
}

/// 购物车查询参数
#[derive(Deserialize)]
pub struct ShoppingCartReportQuery {
    pub user_id: Option<String>,
}

/// 菜品完整信息查询参数
#[derive(Deserialize)]
pub struct DishFullInfoReportQuery {
    pub dish_id: Option<String>,
    pub category_id: Option<String>,
    pub status: Option<i32>,
}

/// 用户订单统计查询参数
#[derive(Deserialize)]
pub struct UserOrderStatsReportQuery {
    pub user_id: Option<String>,
    pub phone: Option<String>,
}

/// 地址簿查询参数
#[derive(Deserialize)]
pub struct AddressBookReportQuery {
    pub user_id: Option<String>,
    pub is_default: Option<i32>,
}

// ==================== 报表接口实现 ====================

/// 订单详情报表
/// 
/// 查询订单及其关联的订单详情信息
/// GET /report/order-details
pub async fn get_order_details_report(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<OrderDetailsReportQuery>,
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
            tracing::error!("查询订单详情报表失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 套餐菜品报表
/// 
/// 查询套餐及其包含的菜品信息
/// GET /report/setmeal-dishes
pub async fn get_setmeal_dishes_report(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<SetmealDishesReportQuery>,
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
            tracing::error!("查询套餐菜品报表失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 购物车商品报表
/// 
/// 查询购物车及关联的商品信息（菜品或套餐）
/// GET /report/shopping-cart
pub async fn get_shopping_cart_report(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<ShoppingCartReportQuery>,
) -> Json<ApiResponse<Vec<crate::domain::dto::join_dto::ShoppingCartWithItemDTO>>> {
    match JoinQuery::select_shopping_cart_with_items(
        rb,
        params.user_id.as_deref(),
    )
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => {
            tracing::error!("查询购物车报表失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 菜品完整信息报表
/// 
/// 查询菜品及其分类、口味信息
/// GET /report/dish-full-info
pub async fn get_dish_full_info_report(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<DishFullInfoReportQuery>,
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
            tracing::error!("查询菜品信息报表失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 用户订单统计报表
/// 
/// 查询用户及其订单统计数据
/// GET /report/user-order-stats
pub async fn get_user_order_stats_report(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<UserOrderStatsReportQuery>,
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
            tracing::error!("查询用户统计报表失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}

/// 地址簿用户报表
/// 
/// 查询地址簿及关联的用户信息
/// GET /report/address-book
pub async fn get_address_book_report(
    Extension(rb): Extension<&'static RBatis>,
    Query(params): Query<AddressBookReportQuery>,
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
            tracing::error!("查询地址簿报表失败: {:?}", e);
            Json(ApiResponse::error("查询失败"))
        }
    }
}
