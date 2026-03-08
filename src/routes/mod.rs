use axum::{routing::get, routing::post, Extension, Router, Json};
use rbatis::RBatis;
use crate::handlers::get_handler::{get_handler, get_body_handler};
use crate::handlers::post_handler::post_handler;
use crate::handlers::login_handler::login_handler;
use crate::handlers::logout_handler::logout_handler;
use crate::handlers::paginate_handler::{paginate_handler, paginate_with_params_handler};
use crate::handlers::crud_handler::*;
use crate::middleware::session_validation::validate_session;
use serde_json;
use crate::domain::table::sys_dict::SysDict;

pub fn setup_routes() -> Router {
    Router::new()
        .route("/api/login", post(login_handler))
        .route("/api/logout", post(logout_handler).layer(axum::middleware::from_fn(validate_session)))
        .route("/get", get(get_handler).layer(axum::middleware::from_fn(validate_session)))
        .route("/post", post(post_handler).layer(axum::middleware::from_fn(validate_session)))
        .route("/getBody", post(get_body_handler).layer(axum::middleware::from_fn(validate_session)))
        .route("/paginate", post(paginate_handler).layer(axum::middleware::from_fn(validate_session)))
        .route("/paginate-with-params", post(paginate_with_params_handler).layer(axum::middleware::from_fn(validate_session)))
        // SysDict 相关路由
        .route("/sys-dict/select-page", post(|Extension(rb): Extension<&'static RBatis>, Json(payload): Json<SysDictPageRequest>| async move {
            use axum::response::IntoResponse;
            
            // 打印请求体
            if let Ok(json_str) = serde_json::to_string(&payload) {
                tracing::debug!("Request body for /sys-dict/select-page (JSON): {}", json_str);
            }
            
            // 处理请求
            match sys_dict_select_page(Json(payload), Extension(rb)).await {
                Ok(json_response) => {
                    // 打印响应体
                    if let Ok(json_str) = serde_json::to_string(&json_response.0) {
                        tracing::debug!("Response body for /sys-dict/select-page (JSON): {}", json_str);
                    }
                    json_response.into_response()
                },
                Err((status, message)) => {
                    tracing::error!("Error in sys_dict_select_page: {}, {:?}", status, message);
                    (status, message).into_response()
                }
            }
        }).layer(axum::middleware::from_fn(validate_session)))
        .route("/sys-dict/insert", post(|Extension(rb): Extension<&'static RBatis>, Json(payload): Json<SysDict>| async move {
            use axum::response::IntoResponse;
            use crate::domain::table::sys_dict::SysDict;
            
            // 打印请求体
            if let Ok(json_str) = serde_json::to_string(&payload) {
                tracing::debug!("Request body for /sys-dict/insert (JSON): {}", json_str);
            }
            
            // 处理请求
            match sys_dict_insert(Json(payload), Extension(rb)).await {
                Ok(json_response) => {
                    // 打印响应体
                    if let Ok(json_str) = serde_json::to_string(&json_response.0) {
                        tracing::debug!("Response body for /sys-dict/insert (JSON): {}", json_str);
                    }
                    json_response.into_response()
                },
                Err((status, message)) => {
                    tracing::error!("Error in sys_dict_insert: {}, {:?}", status, message);
                    (status, message).into_response()
                }
            }
        }).layer(axum::middleware::from_fn(validate_session)))
        .route("/sys-dict/update", post(|Extension(rb): Extension<&'static RBatis>, Json(payload): Json<SysDict>| async move {
            use axum::response::IntoResponse;
            use crate::domain::table::sys_dict::SysDict;
            
            // 打印请求体
            if let Ok(json_str) = serde_json::to_string(&payload) {
                tracing::debug!("Request body for /sys-dict/update (JSON): {}", json_str);
            }
            
            // 处理请求
            match sys_dict_update(Json(payload), Extension(rb)).await {
                Ok(json_response) => {
                    // 打印响应体
                    if let Ok(json_str) = serde_json::to_string(&json_response.0) {
                        tracing::debug!("Response body for /sys-dict/update (JSON): {}", json_str);
                    }
                    json_response.into_response()
                },
                Err((status, message)) => {
                    tracing::error!("Error in sys_dict_update: {}, {:?}", status, message);
                    (status, message).into_response()
                }
            }
        }).layer(axum::middleware::from_fn(validate_session)))
        .route("/sys-dict/delete", post(|Extension(rb): Extension<&'static RBatis>, Json(payload): Json<String>| async move {
            use axum::response::IntoResponse;
            
            // 打印请求体
            tracing::debug!("Request body for /sys-dict/delete (JSON): {}", payload);
            
            // 处理请求
            match sys_dict_delete(Json(payload), Extension(rb)).await {
                Ok(json_response) => {
                    // 打印响应体
                    if let Ok(json_str) = serde_json::to_string(&json_response.0) {
                        tracing::debug!("Response body for /sys-dict/delete (JSON): {}", json_str);
                    }
                    json_response.into_response()
                },
                Err((status, message)) => {
                    tracing::error!("Error in sys_dict_delete: {}, {:?}", status, message);
                    (status, message).into_response()
                }
            }
        }).layer(axum::middleware::from_fn(validate_session)))
        .route("/api/sys-dict/list", post(|Extension(rb): Extension<&'static RBatis>, Json(payload): Json<serde_json::Value>| async move {
            use crate::domain::table::sys_dict::SysDict;
            use axum::response::IntoResponse;

            // 打印请求体
            tracing::debug!("Request body for /api/sys-dict/list (JSON): {}", payload);

            // 处理请求
            match SysDict::select_all(rb).await {
                Ok(dicts) => {
                    // 打印响应体
                    if let Ok(json_str) = serde_json::to_string(&dicts) {
                        tracing::debug!("Response body for /api/sys-dict/list (JSON): {}", json_str);
                    }
                    axum::Json(dicts).into_response()
                },
                Err(e) => {
                    tracing::error!("Failed to get sys_dict list: {:?}", e);
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }).layer(axum::middleware::from_fn(validate_session)))
        // AddressBook 相关路由
        .route("/address-book/select-page", post(address_book_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/address-book/insert", post(address_book_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/address-book/update", post(address_book_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/address-book/delete", post(address_book_delete).layer(axum::middleware::from_fn(validate_session)))
        // Category 相关路由
        .route("/category/select-page", post(category_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/category/insert", post(category_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/category/update", post(category_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/category/delete", post(category_delete).layer(axum::middleware::from_fn(validate_session)))
        // Dish 相关路由
        .route("/dish/select-page", post(dish_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish/insert", post(dish_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish/update", post(dish_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish/delete", post(dish_delete).layer(axum::middleware::from_fn(validate_session)))
        // DishFlavor 相关路由
        .route("/dish-flavor/select-page", post(dish_flavor_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish-flavor/insert", post(dish_flavor_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish-flavor/update", post(dish_flavor_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish-flavor/delete", post(dish_flavor_delete).layer(axum::middleware::from_fn(validate_session)))
        // Employee 相关路由
        .route("/employee/select-page", post(employee_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/employee/insert", post(employee_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/employee/update", post(employee_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/employee/delete", post(employee_delete).layer(axum::middleware::from_fn(validate_session)))
        // OrderDetail 相关路由
        .route("/order-detail/select-page", post(order_detail_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/order-detail/insert", post(order_detail_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/order-detail/update", post(order_detail_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/order-detail/delete", post(order_detail_delete).layer(axum::middleware::from_fn(validate_session)))
        // Orders 相关路由
        .route("/orders/select-page", post(orders_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/orders/insert", post(orders_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/orders/update", post(orders_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/orders/delete", post(orders_delete).layer(axum::middleware::from_fn(validate_session)))
        // Setmeal 相关路由
        .route("/setmeal/select-page", post(setmeal_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal/insert", post(setmeal_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal/update", post(setmeal_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal/delete", post(setmeal_delete).layer(axum::middleware::from_fn(validate_session)))
        // SetmealDish 相关路由
        .route("/setmeal-dish/select-page", post(setmeal_dish_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal-dish/insert", post(setmeal_dish_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal-dish/update", post(setmeal_dish_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal-dish/delete", post(setmeal_dish_delete).layer(axum::middleware::from_fn(validate_session)))
        // ShoppingCart 相关路由
        .route("/shopping-cart/select-page", post(shopping_cart_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/shopping-cart/insert", post(shopping_cart_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/shopping-cart/update", post(shopping_cart_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/shopping-cart/delete", post(shopping_cart_delete).layer(axum::middleware::from_fn(validate_session)))
        // User 相关路由
        .route("/user/select-page", post(user_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/user/insert", post(user_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/user/update", post(user_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/user/delete", post(user_delete).layer(axum::middleware::from_fn(validate_session)))
}