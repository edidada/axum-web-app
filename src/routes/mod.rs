use crate::domain::table::sys_dict::SysDict;
use crate::handlers::crud_handler;
use crate::handlers::crud_handler::SysDictPageRequest;
use crate::handlers::get_handler::{get_body_handler, get_handler};
use crate::handlers::health_handler::{
    health_check, health_check_detailed, liveness_check, readiness_check,
};
#[cfg(feature = "minio")]
use crate::handlers::image_handler::{delete_image, get_image_url, upload_image_from_url};
#[cfg(feature = "minio")]
use crate::handlers::image_sync_handler::sync_images_to_minio;
use crate::handlers::login_handler::login_handler;
use crate::handlers::logout_handler::logout_handler;
use crate::handlers::paginate_handler::{paginate_handler, paginate_with_params_handler};
use crate::handlers::post_handler::post_handler;
use crate::handlers::report_handler;
use crate::middleware::session_validation::validate_session;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use rbatis::RBatis;
use serde_json;

pub fn setup_routes() -> Router {
    let router = Router::new()
        // 健康检查路由
        .route("/health", get(health_check))
        .route("/health/detailed", get(health_check_detailed))
        .route("/ready", get(readiness_check))
        .route("/live", get(liveness_check))
        // 认证路由
        .route("/api/login", post(login_handler))
        .route("/api/logout", post(logout_handler).layer(axum::middleware::from_fn(validate_session)))
        // 保留的旧路由（兼容）
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
            match crud_handler::sys_dict_select_page(Json(payload), Extension(rb)).await {
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

            // 打印请求体
            if let Ok(json_str) = serde_json::to_string(&payload) {
                tracing::debug!("Request body for /sys-dict/insert (JSON): {}", json_str);
            }

            // 处理请求
            match crud_handler::sys_dict_insert(Json(payload), Extension(rb)).await {
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

            // 打印请求体
            if let Ok(json_str) = serde_json::to_string(&payload) {
                tracing::debug!("Request body for /sys-dict/update (JSON): {}", json_str);
            }

            // 处理请求
            match crud_handler::sys_dict_update(Json(payload), Extension(rb)).await {
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
            match crud_handler::sys_dict_delete(Json(payload), Extension(rb)).await {
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
            use crate::domain::dto::dict_dto::DictPageDTO;
            use rbatis::PageRequest;
            use axum::response::IntoResponse;

            // 打印请求体
            tracing::debug!("Request body for /api/sys-dict/list (JSON): {}", payload);

            // 解析查询参数
            let code = payload.get("code").and_then(|v| v.as_str()).map(|s| s.to_string());
            let name = payload.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());

            let dto = DictPageDTO {
                code,
                name,
            };

            let page_request = PageRequest::new(1, 1000); // 默认查询前1000条

            // 处理请求 - 使用分页查询支持条件筛选
            match SysDict::select_page(rb, &page_request, &dto).await {
                Ok(page) => {
                    // 打印响应体
                    if let Ok(json_str) = serde_json::to_string(&page.records) {
                        tracing::debug!("Response body for /api/sys-dict/list (JSON): {}", json_str);
                    }
                    axum::Json(page.records).into_response()
                },
                Err(e) => {
                    tracing::error!("Failed to get sys_dict list: {:?}", e);
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }).layer(axum::middleware::from_fn(validate_session)))
        // AddressBook 相关路由
        .route("/address-book/select-page", post(crud_handler::address_book_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/address-book/insert", post(crud_handler::address_book_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/address-book/update", post(crud_handler::address_book_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/address-book/delete", post(crud_handler::address_book_delete).layer(axum::middleware::from_fn(validate_session)))
        // Category 相关路由
        .route("/category/select-page", post(crud_handler::category_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/category/insert", post(crud_handler::category_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/category/update", post(crud_handler::category_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/category/delete", post(crud_handler::category_delete).layer(axum::middleware::from_fn(validate_session)))
        // Dish 相关路由
        .route("/dish/select-page", post(crud_handler::dish_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish/insert", post(crud_handler::dish_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish/update", post(crud_handler::dish_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish/delete", post(crud_handler::dish_delete).layer(axum::middleware::from_fn(validate_session)))
        // DishFlavor 相关路由
        .route("/dish-flavor/select-page", post(crud_handler::dish_flavor_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish-flavor/insert", post(crud_handler::dish_flavor_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish-flavor/update", post(crud_handler::dish_flavor_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/dish-flavor/delete", post(crud_handler::dish_flavor_delete).layer(axum::middleware::from_fn(validate_session)))
        // Employee 相关路由
        .route("/employee/select-page", post(crud_handler::employee_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/employee/insert", post(crud_handler::employee_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/employee/update", post(crud_handler::employee_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/employee/delete", post(crud_handler::employee_delete).layer(axum::middleware::from_fn(validate_session)))
        // OrderDetail 相关路由
        .route("/order-detail/select-page", post(crud_handler::order_detail_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/order-detail/insert", post(crud_handler::order_detail_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/order-detail/update", post(crud_handler::order_detail_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/order-detail/delete", post(crud_handler::order_detail_delete).layer(axum::middleware::from_fn(validate_session)))
        // Orders 相关路由
        .route("/orders/select-page", post(crud_handler::orders_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/orders/insert", post(crud_handler::orders_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/orders/update", post(crud_handler::orders_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/orders/delete", post(crud_handler::orders_delete).layer(axum::middleware::from_fn(validate_session)))
        // Setmeal 相关路由
        .route("/setmeal/select-page", post(crud_handler::setmeal_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal/insert", post(crud_handler::setmeal_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal/update", post(crud_handler::setmeal_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal/delete", post(crud_handler::setmeal_delete).layer(axum::middleware::from_fn(validate_session)))
        // SetmealDish 相关路由
        .route("/setmeal-dish/select-page", post(crud_handler::setmeal_dish_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal-dish/insert", post(crud_handler::setmeal_dish_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal-dish/update", post(crud_handler::setmeal_dish_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/setmeal-dish/delete", post(crud_handler::setmeal_dish_delete).layer(axum::middleware::from_fn(validate_session)))
        // ShoppingCart 相关路由
        .route("/shopping-cart/select-page", post(crud_handler::shopping_cart_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/shopping-cart/insert", post(crud_handler::shopping_cart_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/shopping-cart/update", post(crud_handler::shopping_cart_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/shopping-cart/delete", post(crud_handler::shopping_cart_delete).layer(axum::middleware::from_fn(validate_session)))
        // User 相关路由
        .route("/user/select-page", post(crud_handler::user_select_page).layer(axum::middleware::from_fn(validate_session)))
        .route("/user/insert", post(crud_handler::user_insert).layer(axum::middleware::from_fn(validate_session)))
        .route("/user/update", post(crud_handler::user_update).layer(axum::middleware::from_fn(validate_session)))
        .route("/user/delete", post(crud_handler::user_delete).layer(axum::middleware::from_fn(validate_session)))
        // 报表查询路由 (原 JOIN 查询)
        .route("/report/order-details", get(report_handler::get_order_details_report).layer(axum::middleware::from_fn(validate_session)))
        .route("/report/setmeal-dishes", get(report_handler::get_setmeal_dishes_report).layer(axum::middleware::from_fn(validate_session)))
        .route("/report/shopping-cart", get(report_handler::get_shopping_cart_report).layer(axum::middleware::from_fn(validate_session)))
        .route("/report/dish-full-info", get(report_handler::get_dish_full_info_report).layer(axum::middleware::from_fn(validate_session)))
        .route("/report/user-order-stats", get(report_handler::get_user_order_stats_report).layer(axum::middleware::from_fn(validate_session)))
        .route("/report/address-book", get(report_handler::get_address_book_report).layer(axum::middleware::from_fn(validate_session)));

    #[cfg(feature = "minio")]
    {
        router
            .route(
                "/api/image/upload-url",
                get(upload_image_from_url).layer(axum::middleware::from_fn(validate_session)),
            )
            .route(
                "/api/image/:object_name",
                get(get_image_url).layer(axum::middleware::from_fn(validate_session)),
            )
            .route(
                "/api/image/:object_name",
                post(delete_image).layer(axum::middleware::from_fn(validate_session)),
            )
            .route(
                "/api/image/sync",
                get(sync_images_to_minio).layer(axum::middleware::from_fn(validate_session)),
            )
    }

    #[cfg(not(feature = "minio"))]
    router
}
