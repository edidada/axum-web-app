-- PostgreSQL 数据库建表脚本
-- 数据库: ruiji
-- 用户: ibqo

-- 字典表
CREATE TABLE IF NOT EXISTS sys_dict (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    code VARCHAR(255),
    state INTEGER,
    create_date TIMESTAMP
);

-- 地址簿表
CREATE TABLE IF NOT EXISTS address_book (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255),
    consignee VARCHAR(255),
    sex INTEGER,
    phone VARCHAR(255),
    province_code VARCHAR(255),
    province_name VARCHAR(255),
    city_code VARCHAR(255),
    city_name VARCHAR(255),
    district_code VARCHAR(255),
    district_name VARCHAR(255),
    detail VARCHAR(255),
    label VARCHAR(255),
    is_default INTEGER,
    create_time TIMESTAMP,
    update_time TIMESTAMP,
    create_user VARCHAR(255),
    update_user VARCHAR(255),
    is_deleted INTEGER DEFAULT 0
);

-- 分类表
CREATE TABLE IF NOT EXISTS category (
    id VARCHAR(255) PRIMARY KEY,
    category_type INTEGER,
    name VARCHAR(255),
    sort INTEGER,
    create_time TIMESTAMP,
    update_time TIMESTAMP,
    create_user VARCHAR(255),
    update_user VARCHAR(255)
);

-- 菜品表
CREATE TABLE IF NOT EXISTS dish (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    category_id VARCHAR(255),
    price VARCHAR(255),
    code VARCHAR(255),
    image VARCHAR(255),
    description VARCHAR(255),
    status INTEGER,
    sort INTEGER,
    create_time TIMESTAMP,
    update_time TIMESTAMP,
    create_user VARCHAR(255),
    update_user VARCHAR(255),
    is_deleted INTEGER DEFAULT 0
);

-- 菜品口味表
CREATE TABLE IF NOT EXISTS dish_flavor (
    id VARCHAR(255) PRIMARY KEY,
    dish_id VARCHAR(255),
    name VARCHAR(255),
    value VARCHAR(255),
    create_time TIMESTAMP,
    update_time TIMESTAMP,
    create_user VARCHAR(255),
    update_user VARCHAR(255),
    is_deleted INTEGER DEFAULT 0
);

-- 员工表
CREATE TABLE IF NOT EXISTS employee (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    username VARCHAR(255),
    password VARCHAR(255),
    phone VARCHAR(255),
    sex INTEGER,
    id_number VARCHAR(255),
    status INTEGER,
    create_time TIMESTAMP,
    update_time TIMESTAMP,
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
    number INTEGER,
    amount VARCHAR(255),
    create_time TIMESTAMP
);

-- 订单表
CREATE TABLE IF NOT EXISTS orders (
    id VARCHAR(255) PRIMARY KEY,
    number VARCHAR(255),
    status INTEGER,
    user_id VARCHAR(255),
    address_book_id VARCHAR(255),
    order_time TIMESTAMP,
    checkout_time TIMESTAMP,
    pay_method INTEGER,
    pay_status INTEGER,
    amount VARCHAR(255),
    remark VARCHAR(255),
    phone VARCHAR(255),
    address VARCHAR(255),
    user_name VARCHAR(255),
    consignee VARCHAR(255),
    create_time TIMESTAMP,
    update_time TIMESTAMP
);

-- 套餐表
CREATE TABLE IF NOT EXISTS setmeal (
    id VARCHAR(255) PRIMARY KEY,
    category_id VARCHAR(255),
    name VARCHAR(255),
    price VARCHAR(255),
    status INTEGER,
    code VARCHAR(255),
    description VARCHAR(255),
    image VARCHAR(255),
    create_time TIMESTAMP,
    update_time TIMESTAMP,
    create_user VARCHAR(255),
    update_user VARCHAR(255),
    is_deleted INTEGER DEFAULT 0
);

-- 套餐菜品关联表
CREATE TABLE IF NOT EXISTS setmeal_dish (
    id VARCHAR(255) PRIMARY KEY,
    setmeal_id VARCHAR(255),
    dish_id VARCHAR(255),
    name VARCHAR(255),
    price VARCHAR(255),
    copies INTEGER,
    create_time TIMESTAMP,
    update_time TIMESTAMP,
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
    number INTEGER,
    amount VARCHAR(255),
    create_time TIMESTAMP
);

-- 用户表
CREATE TABLE IF NOT EXISTS "user" (
    id VARCHAR(255) PRIMARY KEY,
    openid VARCHAR(255),
    name VARCHAR(255),
    phone VARCHAR(255),
    sex INTEGER,
    id_number VARCHAR(255),
    avatar VARCHAR(255),
    create_time TIMESTAMP,
    update_time TIMESTAMP
);
