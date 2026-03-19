// JOIN 查询 DTO

// 订单详情 JOIN 查询
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderWithDetailDTO {
    // 订单信息
    pub order_id: Option<String>,
    pub order_number: Option<String>,
    pub order_status: Option<i32>,
    pub user_id: Option<String>,
    pub order_time: Option<String>,
    pub pay_method: Option<i32>,
    pub pay_status: Option<i32>,
    pub order_amount: Option<f64>,
    pub order_remark: Option<String>,
    pub consignee: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    // 订单详情信息
    pub detail_id: Option<String>,
    pub dish_id: Option<String>,
    pub setmeal_id: Option<String>,
    pub dish_name: Option<String>,
    pub dish_image: Option<String>,
    pub dish_flavor: Option<String>,
    pub number: Option<i32>,
    pub amount: Option<f64>,
}

// 套餐 JOIN 菜品查询
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetmealWithDishDTO {
    // 套餐信息
    pub setmeal_id: Option<String>,
    pub setmeal_name: Option<String>,
    pub setmeal_price: Option<f64>,
    pub setmeal_status: Option<i32>,
    pub setmeal_code: Option<String>,
    pub setmeal_description: Option<String>,
    pub setmeal_image: Option<String>,
    // 分类信息
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    // 套餐菜品关联信息
    pub sd_id: Option<String>,
    pub copies: Option<i32>,
    // 菜品信息
    pub dish_id: Option<String>,
    pub dish_name: Option<String>,
    pub dish_price: Option<f64>,
}

// 购物车 JOIN 商品信息查询
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ShoppingCartWithItemDTO {
    // 购物车信息
    pub cart_id: Option<String>,
    pub user_id: Option<String>,
    pub dish_flavor: Option<String>,
    pub number: Option<i32>,
    pub cart_amount: Option<f64>,
    pub create_time: Option<String>,
    // 商品信息（可能是菜品或套餐）
    pub item_id: Option<String>,
    pub item_name: Option<String>,
    pub item_image: Option<String>,
    pub item_price: Option<f64>,
    pub item_type: Option<String>, // "dish" 或 "setmeal"
}

// 菜品 JOIN 分类和口味查询
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DishFullInfoDTO {
    // 菜品信息
    pub dish_id: Option<String>,
    pub dish_name: Option<String>,
    pub dish_price: Option<f64>,
    pub dish_code: Option<String>,
    pub dish_image: Option<String>,
    pub dish_description: Option<String>,
    pub dish_status: Option<i32>,
    pub dish_sort: Option<i32>,
    // 分类信息
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    // 口味信息（可能有多个，用逗号分隔）
    pub flavors: Option<String>,
}

// 员工 JOIN 创建人/更新人查询
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EmployeeWithOperatorDTO {
    // 员工信息
    pub id: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<i32>,
    pub id_number: Option<String>,
    pub status: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    // 创建人信息
    pub create_user_name: Option<String>,
    // 更新人信息
    pub update_user_name: Option<String>,
}

// 用户 JOIN 订单统计查询
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserWithOrderStatsDTO {
    // 用户信息
    pub user_id: Option<String>,
    pub openid: Option<String>,
    pub user_name: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<i32>,
    // 订单统计
    pub order_count: Option<i64>,
    pub total_amount: Option<f64>,
}
