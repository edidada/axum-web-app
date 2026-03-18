use axum::{response::Json, extract::Extension, http::StatusCode};
use rbatis::rbatis::RBatis;
use rbatis::PageRequest;
use rbs::{value};
use serde::{Serialize, Deserialize};

use crate::domain::table::sys_dict::SysDict;
use crate::domain::table::address_book::AddressBook;
use crate::domain::table::category::Category;
use crate::domain::table::dish::Dish;
use crate::domain::table::dish_flavor::DishFlavor;
use crate::domain::table::employee::Employee;
use crate::domain::table::order_detail::OrderDetail;
use crate::domain::table::orders::Orders;
use crate::domain::table::setmeal::Setmeal;
use crate::domain::table::setmeal_dish::SetmealDish;
use crate::domain::table::shopping_cart::ShoppingCart;
use crate::domain::table::user::User;
use crate::domain::dto::dict_dto::DictPageDTO;
use crate::domain::dto::address_book_dto::AddressBookPageDTO;
use crate::domain::dto::category_dto::CategoryPageDTO;
use crate::domain::dto::dish_dto::DishPageDTO;
use crate::domain::dto::dish_flavor_dto::DishFlavorPageDTO;
use crate::domain::dto::employee_dto::EmployeePageDTO;
use crate::domain::dto::order_detail_dto::OrderDetailPageDTO;
use crate::domain::dto::orders_dto::OrdersPageDTO;
use crate::domain::dto::setmeal_dto::SetmealPageDTO;
use crate::domain::dto::setmeal_dish_dto::SetmealDishPageDTO;
use crate::domain::dto::shopping_cart_dto::ShoppingCartPageDTO;
use crate::domain::dto::user_dto::UserPageDTO;
use rand::Rng;
use rbatis::rbdc::db::ExecResult;

fn generate_id() -> String {    let mut rng = rand::thread_rng();    let id: u64 = rng.gen();    id.to_string()}

// 通用分页请求结构体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageQuery {
    pub page: u64,
    pub pageSize: u64,
}

// SysDict 相关接口
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysDictPageRequest {
    pub page: u64,
    pub size: u64,
    pub code: Option<String>,
    pub name: Option<String>,
}

pub async fn sys_dict_select_page(
    Json(payload): Json<SysDictPageRequest>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<SysDict>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.size);

    let dto = DictPageDTO {
        code: payload.code,
        name: payload.name,
    };
    let result = SysDict::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn sys_dict_insert(
    Json(dict): Json<SysDict>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<SysDict>, (StatusCode, String)> {
    let mut dict = dict;
    dict.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = SysDict::insert(rb, &dict).await;
    match result {
        Ok(_) => Ok(Json(dict)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn sys_dict_update(
    Json(dict): Json<SysDict>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<SysDict>, (StatusCode, String)> {
    let id = dict.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少字典 ID".to_string()))?;

    SysDict::update_by_map(
        rb,
        &dict,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(dict))
}

pub async fn sys_dict_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result = SysDict::delete_by_map(rb, value!{ "id": id }).await;  // ← 去掉类型注解
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// AddressBook 相关接口
pub async fn address_book_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<AddressBook>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = AddressBookPageDTO::default();
    let result: std::result::Result<rbatis::Page<AddressBook>, rbatis::Error> = AddressBook::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn address_book_insert(
    Json(address_book): Json<AddressBook>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<AddressBook>, (StatusCode, String)> {
    let mut address_book = address_book;
    address_book.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = AddressBook::insert(rb, &address_book).await;
    match result {
        Ok(_) => Ok(Json(address_book)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn address_book_update(
    Json(address_book): Json<AddressBook>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<AddressBook>, (StatusCode, String)> {
    let id = address_book.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少地址簿 ID".to_string()))?;

    AddressBook::update_by_map(
        rb,
        &address_book,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(address_book))
}

pub async fn address_book_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result = AddressBook::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// Category 相关接口
pub async fn category_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<Category>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = CategoryPageDTO::default();
    let result: std::result::Result<rbatis::Page<Category>, rbatis::Error> = Category::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn category_insert(
    Json(category): Json<Category>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Category>, (StatusCode, String)> {
    let mut category = category;
    category.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = Category::insert(rb, &category).await;
    match result {
        Ok(_) => Ok(Json(category)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn category_update(
    Json(category): Json<Category>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Category>, (StatusCode, String)> {
    let id = category.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少分类 ID".to_string()))?;

    Category::update_by_map(
        rb,
        &category,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(category))
}

pub async fn category_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result = Category::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// Dish 相关接口
pub async fn dish_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<Dish>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = DishPageDTO::default();
    let result: std::result::Result<rbatis::Page<Dish>, rbatis::Error> = Dish::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn dish_insert(
    Json(dish): Json<Dish>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Dish>, (StatusCode, String)> {
    let mut dish = dish;
    dish.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = Dish::insert(rb, &dish).await;
    match result {
        Ok(_) => Ok(Json(dish)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn dish_update(
    Json(dish): Json<Dish>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Dish>, (StatusCode, String)> {
    let id = dish.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少菜品 ID".to_string()))?;

    Dish::update_by_map(
        rb,
        &dish,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(dish))
}

pub async fn dish_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result = Dish::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// DishFlavor 相关接口
pub async fn dish_flavor_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<DishFlavor>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = DishFlavorPageDTO::default();
    let result: std::result::Result<rbatis::Page<DishFlavor>, rbatis::Error> = DishFlavor::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn dish_flavor_insert(
    Json(dish_flavor): Json<DishFlavor>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<DishFlavor>, (StatusCode, String)> {
    let mut dish_flavor = dish_flavor;
    dish_flavor.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = DishFlavor::insert(rb, &dish_flavor).await;
    match result {
        Ok(_) => Ok(Json(dish_flavor)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn dish_flavor_update(
    Json(dish_flavor): Json<DishFlavor>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<DishFlavor>, (StatusCode, String)> {
    let id = dish_flavor.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少菜品口味 ID".to_string()))?;

    DishFlavor::update_by_map(
        rb,
        &dish_flavor,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(dish_flavor))
}

pub async fn dish_flavor_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result = DishFlavor::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// Employee 相关接口
pub async fn employee_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<Employee>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = EmployeePageDTO::default();
    let result: std::result::Result<rbatis::Page<Employee>, rbatis::Error> = Employee::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn employee_insert(
    Json(employee): Json<Employee>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Employee>, (StatusCode, String)> {
    let mut employee = employee;
    employee.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = Employee::insert(rb, &employee).await;
    match result {
        Ok(_) => Ok(Json(employee)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn employee_update(
    Json(employee): Json<Employee>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Employee>, (StatusCode, String)> {
    let id = employee.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少员工 ID".to_string()))?;

    Employee::update_by_map(
        rb,
        &employee,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(employee))
}

pub async fn employee_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result = Employee::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// OrderDetail 相关接口
pub async fn order_detail_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<OrderDetail>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = OrderDetailPageDTO::default();
    let result: std::result::Result<rbatis::Page<OrderDetail>, rbatis::Error> = OrderDetail::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn order_detail_insert(
    Json(order_detail): Json<OrderDetail>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<OrderDetail>, (StatusCode, String)> {
    let mut order_detail = order_detail;
    order_detail.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = OrderDetail::insert(rb, &order_detail).await;
    match result {
        Ok(_) => Ok(Json(order_detail)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn order_detail_update(
    Json(order_detail): Json<OrderDetail>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<OrderDetail>, (StatusCode, String)> {
    let id = order_detail.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少订单详情 ID".to_string()))?;

    OrderDetail::update_by_map(
        rb,
        &order_detail,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(order_detail))
}

pub async fn order_detail_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result = OrderDetail::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// Orders 相关接口
pub async fn orders_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<Orders>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = OrdersPageDTO::default();
    let result: std::result::Result<rbatis::Page<Orders>, rbatis::Error> = Orders::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn orders_insert(
    Json(orders): Json<Orders>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Orders>, (StatusCode, String)> {
    let mut orders = orders;
    orders.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = Orders::insert(rb, &orders).await;
    match result {
        Ok(_) => Ok(Json(orders)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn orders_update(
    Json(orders): Json<Orders>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Orders>, (StatusCode, String)> {
    let id = orders.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少订单 ID".to_string()))?;

    Orders::update_by_map(
        rb,
        &orders,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(orders))
}

pub async fn orders_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result: std::result::Result<ExecResult, rbatis::Error> = Orders::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// Setmeal 相关接口
pub async fn setmeal_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<Setmeal>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = SetmealPageDTO::default();
    let result: std::result::Result<rbatis::Page<Setmeal>, rbatis::Error> = Setmeal::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn setmeal_insert(
    Json(setmeal): Json<Setmeal>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Setmeal>, (StatusCode, String)> {
    let mut setmeal = setmeal;
    setmeal.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = Setmeal::insert(rb, &setmeal).await;
    match result {
        Ok(_) => Ok(Json(setmeal)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn setmeal_update(
    Json(setmeal): Json<Setmeal>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<Setmeal>, (StatusCode, String)> {
    let id = setmeal.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少套餐 ID".to_string()))?;

    Setmeal::update_by_map(
        rb,
        &setmeal,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(setmeal))
}

pub async fn setmeal_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result: std::result::Result<ExecResult, rbatis::Error> = Setmeal::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// SetmealDish 相关接口
pub async fn setmeal_dish_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<SetmealDish>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = SetmealDishPageDTO::default();
    let result: std::result::Result<rbatis::Page<SetmealDish>, rbatis::Error> = SetmealDish::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn setmeal_dish_insert(
    Json(setmeal_dish): Json<SetmealDish>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<SetmealDish>, (StatusCode, String)> {
    let mut setmeal_dish = setmeal_dish;
    setmeal_dish.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = SetmealDish::insert(rb, &setmeal_dish).await;
    match result {
        Ok(_) => Ok(Json(setmeal_dish)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn setmeal_dish_update(
    Json(setmeal_dish): Json<SetmealDish>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<SetmealDish>, (StatusCode, String)> {
    let id = setmeal_dish.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少套餐菜品 ID".to_string()))?;

    SetmealDish::update_by_map(
        rb,
        &setmeal_dish,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(setmeal_dish))
}

pub async fn setmeal_dish_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result: std::result::Result<ExecResult, rbatis::Error> = SetmealDish::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// ShoppingCart 相关接口
pub async fn shopping_cart_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<ShoppingCart>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = ShoppingCartPageDTO::default();
    let result: std::result::Result<rbatis::Page<ShoppingCart>, rbatis::Error> = ShoppingCart::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn shopping_cart_insert(
    Json(shopping_cart): Json<ShoppingCart>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<ShoppingCart>, (StatusCode, String)> {
    let mut shopping_cart = shopping_cart;
    shopping_cart.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = ShoppingCart::insert(rb, &shopping_cart).await;
    match result {
        Ok(_) => Ok(Json(shopping_cart)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn shopping_cart_update(
    Json(shopping_cart): Json<ShoppingCart>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<ShoppingCart>, (StatusCode, String)> {
    let id = shopping_cart.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少购物车 ID".to_string()))?;

    ShoppingCart::update_by_map(
        rb,
        &shopping_cart,
        value!{ "id": id }
        // 如果要限制字段，可加 "column": ["quantity", "status", ...]
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(shopping_cart))
}

pub async fn shopping_cart_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result: std::result::Result<ExecResult, rbatis::Error> = ShoppingCart::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// User 相关接口
pub async fn user_select_page(
    Json(payload): Json<PageQuery>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<rbatis::Page<User>>, (StatusCode, String)> {
    let page_request = PageRequest::new(payload.page, payload.pageSize);
    let dto = UserPageDTO::default();
    let result: std::result::Result<rbatis::Page<User>, rbatis::Error> = User::select_page(rb, &page_request, &dto).await;
    match result {
        Ok(page) => Ok(Json(page)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn user_insert(
    Json(user): Json<User>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut user = user;
    user.id = Some(generate_id());
    let result: std::result::Result<ExecResult, rbatis::Error> = User::insert(rb, &user).await;
    match result {
        Ok(_) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn user_update(
    Json(user): Json<User>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<User>, (StatusCode, String)> {
    let id = user.id.as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "缺少用户 ID".to_string()))?;

    User::update_by_map(
        rb,
        &user,
        value!{ "id": id }
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库更新失败: {:?}", e)))?;

    Ok(Json(user))
}

pub async fn user_delete(
    Json(id): Json<String>,
    Extension(rb): Extension<&'static RBatis>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result: std::result::Result<ExecResult, rbatis::Error> = User::delete_by_map(rb, value!{ "id": id }).await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}
