use serde::{Deserialize, Serialize};

// 用户结构体，对应 users 表
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub name: Option<String>,
}