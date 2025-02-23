use serde::{Deserialize, Serialize};

// 订单结构体，对应 orders 表
#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub amount: Option<f64>,
}

// 用于存储 LEFT JOIN 查询结果的结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct UserOrder {
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub order_id: Option<i64>,
    pub order_amount: Option<f64>,
}