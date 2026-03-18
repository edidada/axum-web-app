-- PostgreSQL 数据库测试数据插入脚本

-- 字典表数据
INSERT INTO sys_dict (id, name, code, state, create_date) VALUES
('1', '字典项1', 'dict_001', 1, NOW()),
('2', '字典项2', 'dict_002', 1, NOW()),
('3', '字典项3', 'dict_003', 0, NOW()),
('4', '用户状态', 'user_status', 1, NOW()),
('5', '订单状态', 'order_status', 1, NOW());

-- 地址簿表数据
INSERT INTO address_book (id, user_id, consignee, phone, province_name, city_name, district_name, detail, label, is_default, create_time) VALUES
('1', 'user1', '张三', '13800138000', '北京市', '北京市', '朝阳区', '某某街道1号', '家', 1, NOW()),
('2', 'user1', '李四', '13900139000', '上海市', '上海市', '浦东新区', '某某路2号', '公司', 0, NOW()),
('3', 'user2', '王五', '13700137000', '广州市', '广州市', '天河区', '某某大道3号', '家', 1, NOW());

-- 分类表数据
INSERT INTO category (id, category_type, name, sort, create_time) VALUES
('1', 1, '川菜', 1, NOW()),
('2', 1, '粤菜', 2, NOW()),
('3', 1, '湘菜', 3, NOW()),
('4', 2, '主食', 1, NOW()),
('5', 2, '饮品', 2, NOW());

-- 菜品表数据
INSERT INTO dish (id, name, category_id, price, code, description, status, sort, create_time) VALUES
('1', '宫保鸡丁', '1', 38.00, 'D001', '经典川菜，麻辣鲜香', 1, 1, NOW()),
('2', '麻婆豆腐', '1', 22.00, 'D002', '四川传统名菜', 1, 2, NOW()),
('3', '白切鸡', '2', 48.00, 'D003', '粤菜经典，鲜嫩可口', 1, 3, NOW()),
('4', '剁椒鱼头', '3', 68.00, 'D004', '湘菜代表，香辣开胃', 1, 4, NOW()),
('5', '扬州炒饭', '4', 28.00, 'D005', '经典主食', 1, 5, NOW());

-- 菜品口味表数据
INSERT INTO dish_flavor (id, dish_id, name, value, create_time) VALUES
('1', '1', '辣度', '微辣,中辣,特辣', NOW()),
('2', '1', '甜度', '微甜,正常', NOW()),
('3', '2', '辣度', '微辣,中辣', NOW()),
('4', '3', '口味', '原味,酱油', NOW()),
('5', '4', '辣度', '中辣,特辣', NOW());

-- 员工表数据
INSERT INTO employee (id, name, username, password, phone, sex, id_number, status, create_time) VALUES
('1', '管理员', 'admin', 'admin123', '13800138000', 1, '110101199001011234', 1, NOW()),
('2', '张三', 'zhangsan', '123456', '13900139000', 1, '110101199002021234', 1, NOW()),
('3', '李四', 'lisi', '123456', '13700137000', 2, '110101199003031234', 1, NOW());

-- 用户表数据
INSERT INTO "user" (id, openid, name, phone, sex, create_time) VALUES
('1', 'openid_001', '用户1', '13800138001', 1, NOW()),
('2', 'openid_002', '用户2', '13900139001', 2, NOW()),
('3', 'openid_003', '用户3', '13700137001', 1, NOW());

-- 购物车表数据
INSERT INTO shopping_cart (id, name, image, user_id, dish_id, number, amount, create_time) VALUES
('1', '宫保鸡丁', 'dish1.jpg', '1', '1', 2, 76.00, NOW()),
('2', '白切鸡', 'dish3.jpg', '1', '3', 1, 48.00, NOW()),
('3', '扬州炒饭', 'dish5.jpg', '2', '5', 3, 84.00, NOW());

-- 套餐表数据
INSERT INTO setmeal (id, category_id, name, price, status, code, description, create_time) VALUES
('1', '1', '川菜套餐A', 88.00, 1, 'S001', '宫保鸡丁+麻婆豆腐+米饭', NOW()),
('2', '2', '粤菜套餐B', 128.00, 1, 'S002', '白切鸡+时蔬+米饭', NOW()),
('3', '3', '湘菜套餐C', 98.00, 1, 'S003', '剁椒鱼头+米饭', NOW());

-- 套餐菜品关联表数据
INSERT INTO setmeal_dish (id, setmeal_id, dish_id, name, price, copies, create_time) VALUES
('1', '1', '1', '宫保鸡丁', 38.00, 1, NOW()),
('2', '1', '2', '麻婆豆腐', 22.00, 1, NOW()),
('3', '2', '3', '白切鸡', 48.00, 1, NOW()),
('4', '3', '4', '剁椒鱼头', 68.00, 1, NOW());

-- 订单表数据
INSERT INTO orders (id, number, status, user_id, address_book_id, order_time, pay_method, pay_status, amount, remark, phone, address, consignee, create_time) VALUES
('1', 'O202403190001', 1, '1', '1', NOW(), 1, 1, 124.00, '请尽快送达', '13800138000', '北京市朝阳区某某街道1号', '张三', NOW()),
('2', 'O202403190002', 2, '2', '3', NOW(), 2, 1, 48.00, '不要辣', '13700137000', '广州市天河区某某大道3号', '王五', NOW());

-- 订单详情表数据
INSERT INTO order_detail (id, name, image, order_id, dish_id, number, amount, create_time) VALUES
('1', '宫保鸡丁', 'dish1.jpg', '1', '1', 2, 76.00, NOW()),
('2', '麻婆豆腐', 'dish2.jpg', '1', '2', 1, 22.00, NOW()),
('3', '扬州炒饭', 'dish5.jpg', '1', '5', 1, 26.00, NOW()),
('4', '白切鸡', 'dish3.jpg', '2', '3', 1, 48.00, NOW());
