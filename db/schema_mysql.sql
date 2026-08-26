-- MySQL 数据库建表脚本
-- 数据库: ruiji

-- 字典表
CREATE TABLE IF NOT EXISTS sys_dict (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    code VARCHAR(255),
    state INT,
    create_date DATETIME
);

-- 地址簿表
CREATE TABLE IF NOT EXISTS address_book (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255),
    consignee VARCHAR(255),
    sex INT,
    phone VARCHAR(255),
    province_code VARCHAR(255),
    province_name VARCHAR(255),
    city_code VARCHAR(255),
    city_name VARCHAR(255),
    district_code VARCHAR(255),
    district_name VARCHAR(255),
    detail VARCHAR(255),
    label VARCHAR(255),
    is_default INT DEFAULT 0,
    create_time DATETIME,
    update_time DATETIME,
    create_user VARCHAR(255),
    update_user VARCHAR(255),
    is_deleted INT DEFAULT 0
);

-- 分类表
CREATE TABLE IF NOT EXISTS category (
    id VARCHAR(255) PRIMARY KEY,
    category_type INT,
    name VARCHAR(255),
    sort INT,
    create_time DATETIME,
    update_time DATETIME,
    create_user VARCHAR(255),
    update_user VARCHAR(255)
);

-- 菜品表
CREATE TABLE IF NOT EXISTS dish (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    category_id VARCHAR(255),
    price DECIMAL(10,2),
    code VARCHAR(255),
    image VARCHAR(255),
    description VARCHAR(255),
    status INT,
    sort INT,
    create_time DATETIME,
    update_time DATETIME,
    create_user VARCHAR(255),
    update_user VARCHAR(255),
    is_deleted INT DEFAULT 0
);

-- 菜品口味表
CREATE TABLE IF NOT EXISTS dish_flavor (
    id VARCHAR(255) PRIMARY KEY,
    dish_id VARCHAR(255),
    name VARCHAR(255),
    value VARCHAR(255),
    create_time DATETIME,
    update_time DATETIME,
    create_user VARCHAR(255),
    update_user VARCHAR(255),
    is_deleted INT DEFAULT 0
);

-- 员工表
CREATE TABLE IF NOT EXISTS employee (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    username VARCHAR(255),
    password VARCHAR(255),
    phone VARCHAR(255),
    sex INT,
    id_number VARCHAR(255),
    status INT,
    create_time DATETIME,
    update_time DATETIME,
    create_user VARCHAR(255),
    update_user VARCHAR(255)
);

-- 订单详情表
CREATE TABLE IF NOT EXISTS order_detail (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    image VARCHAR(255),
    order_id VARCHAR(255),
    dish_id VARCHAR(255),
    setmeal_id VARCHAR(255),
    dish_flavor VARCHAR(255),
    number INT,
    amount DECIMAL(10,2),
    create_time DATETIME
);

-- 订单表
CREATE TABLE IF NOT EXISTS orders (
    id VARCHAR(255) PRIMARY KEY,
    number VARCHAR(255),
    status INT,
    user_id VARCHAR(255),
    address_book_id VARCHAR(255),
    order_time DATETIME,
    checkout_time DATETIME,
    pay_method INT,
    pay_status INT,
    amount DECIMAL(10,2),
    remark VARCHAR(255),
    phone VARCHAR(255),
    address VARCHAR(255),
    user_name VARCHAR(255),
    consignee VARCHAR(255),
    create_time DATETIME,
    update_time DATETIME
);

-- 套餐表
CREATE TABLE IF NOT EXISTS setmeal (
    id VARCHAR(255) PRIMARY KEY,
    category_id VARCHAR(255),
    name VARCHAR(255),
    price DECIMAL(10,2),
    status INT,
    code VARCHAR(255),
    description VARCHAR(255),
    image VARCHAR(255),
    create_time DATETIME,
    update_time DATETIME,
    create_user VARCHAR(255),
    update_user VARCHAR(255),
    is_deleted INT DEFAULT 0
);

-- 套餐菜品关联表
CREATE TABLE IF NOT EXISTS setmeal_dish (
    id VARCHAR(255) PRIMARY KEY,
    setmeal_id VARCHAR(255),
    dish_id VARCHAR(255),
    name VARCHAR(255),
    price DECIMAL(10,2),
    copies INT,
    create_time DATETIME,
    update_time DATETIME,
    create_user VARCHAR(255),
    update_user VARCHAR(255)
);

-- 购物车表
CREATE TABLE IF NOT EXISTS shopping_cart (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    image VARCHAR(255),
    user_id VARCHAR(255),
    dish_id VARCHAR(255),
    setmeal_id VARCHAR(255),
    dish_flavor VARCHAR(255),
    number INT,
    amount DECIMAL(10,2),
    create_time DATETIME
);

-- 用户表
CREATE TABLE IF NOT EXISTS user (
    id VARCHAR(255) PRIMARY KEY,
    openid VARCHAR(255),
    name VARCHAR(255),
    phone VARCHAR(255),
    sex INT,
    id_number VARCHAR(255),
    avatar VARCHAR(255),
    create_time DATETIME,
    update_time DATETIME
);
-- JWT 会话授权状态、CSRF 绑定与立即撤销。
CREATE TABLE IF NOT EXISTS auth_sessions (
    jti VARCHAR(64) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    role VARCHAR(32) NOT NULL,
    csrf_hash VARCHAR(64) NOT NULL,
    issued_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    revoked_at BIGINT,
    INDEX idx_auth_sessions_expires_at (expires_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
