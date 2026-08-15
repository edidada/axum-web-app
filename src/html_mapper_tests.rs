//! SQLite regression tests for the MyBatis-style mapper files in `domain/table/*.html`.

use std::error::Error;

use rbdc_sqlite::driver::SqliteDriver;
use rbatis::{PageRequest, RBatis};

use crate::domain::dto::{
    address_book_dto::AddressBookPageDTO, category_dto::CategoryPageDTO,
    dict_dto::DictPageDTO, dish_dto::DishPageDTO, dish_flavor_dto::DishFlavorPageDTO,
    employee_dto::EmployeePageDTO, order_detail_dto::OrderDetailPageDTO,
    orders_dto::OrdersPageDTO, setmeal_dish_dto::SetmealDishPageDTO,
    setmeal_dto::SetmealPageDTO, shopping_cart_dto::ShoppingCartPageDTO,
    user_dto::UserPageDTO,
};
use crate::domain::table::{
    address_book::AddressBook, category::Category, dish::Dish, dish_flavor::DishFlavor,
    employee::Employee, join_query::JoinQuery, order_detail::OrderDetail, orders::Orders,
    paginate::PaginateMapper, setmeal::Setmeal, setmeal_dish::SetmealDish,
    shopping_cart::ShoppingCart, sys_dict::SysDict, user::User,
};

async fn test_db() -> RBatis {
    let rb = RBatis::new();
    rb.init(SqliteDriver {}, "sqlite://:memory:").expect("initialize SQLite");

    for statement in [
        "CREATE TABLE address_book (id TEXT, user_id TEXT, consignee TEXT, sex INTEGER, phone TEXT, province_code TEXT, province_name TEXT, city_code TEXT, city_name TEXT, district_code TEXT, district_name TEXT, detail TEXT, label TEXT, is_default INTEGER, create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT, is_deleted INTEGER)",
        "CREATE TABLE category (id TEXT, type INTEGER, name TEXT, sort INTEGER, create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT)",
        "CREATE TABLE dish (id TEXT, name TEXT, category_id TEXT, price TEXT, code TEXT, image TEXT, description TEXT, status INTEGER, sort INTEGER, create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT, is_deleted INTEGER)",
        "CREATE TABLE dish_flavor (id TEXT, dish_id TEXT, name TEXT, value TEXT, create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT, is_deleted INTEGER)",
        "CREATE TABLE employee (id TEXT, name TEXT, username TEXT, password TEXT, phone TEXT, sex INTEGER, id_number TEXT, status INTEGER, create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT)",
        "CREATE TABLE order_detail (id TEXT, name TEXT, image TEXT, order_id TEXT, dish_id TEXT, setmeal_id TEXT, dish_flavor TEXT, number INTEGER, amount TEXT)",
        "CREATE TABLE orders (id TEXT, number TEXT, status INTEGER, user_id TEXT, address_book_id TEXT, order_time TEXT, checkout_time TEXT, pay_method INTEGER, pay_status INTEGER, amount TEXT, remark TEXT, phone TEXT, address TEXT, user_name TEXT, consignee TEXT, cancel_reason TEXT, rejection_reason TEXT, cancel_time TEXT, estimated_delivery_time TEXT, delivery_status INTEGER, delivery_time TEXT, pack_amount INTEGER, tableware_number INTEGER, tableware_status INTEGER)",
        "CREATE TABLE setmeal (id TEXT, category_id TEXT, name TEXT, price TEXT, status INTEGER, code TEXT, description TEXT, image TEXT, create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT, is_deleted INTEGER)",
        "CREATE TABLE setmeal_dish (id TEXT, setmeal_id TEXT, dish_id TEXT, name TEXT, price TEXT, copies INTEGER, sort INTEGER, create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT)",
        "CREATE TABLE shopping_cart (id TEXT, name TEXT, image TEXT, user_id TEXT, dish_id TEXT, setmeal_id TEXT, dish_flavor TEXT, number INTEGER, amount TEXT, create_time TEXT)",
        "CREATE TABLE sys_dict (id TEXT, name TEXT, code TEXT, state INTEGER, create_date TEXT)",
        "CREATE TABLE \"user\" (id TEXT, openid TEXT, name TEXT, phone TEXT, sex INTEGER, id_number TEXT, avatar TEXT, status INTEGER)",
    ] {
        rb.exec(statement, vec![]).await.expect("create test table");
    }
    rb
}

async fn insert(rb: &RBatis, statement: &str) {
    rb.exec(statement, vec![]).await.expect("insert mapper fixture");
}

fn page_request() -> PageRequest {
    PageRequest::new(1, 10)
}

#[tokio::test]
async fn address_book_html_filters_deleted_and_user() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO address_book (id,user_id,consignee,label,is_default,is_deleted) VALUES ('ab-1','u-1','Alice','home',1,0)").await;
    insert(&rb, "INSERT INTO address_book (id,user_id,consignee,label,is_default,is_deleted) VALUES ('ab-2','u-1','Alice','home',1,1)").await;

    let page = AddressBook::select_page(&rb, &page_request(), &AddressBookPageDTO { user_id: Some("u-1".into()), consignee: Some("Alice".into()), phone: None, label: Some("home".into()), is_default: Some(1) }).await?;
    assert_eq!(page.total, 1);
    assert_eq!(page.records[0].id.as_deref(), Some("ab-1"));
    Ok(())
}

#[tokio::test]
async fn category_html_filters_type_and_name() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO category (id,type,name,sort) VALUES ('c-1',1,'Hot dishes',1),('c-2',2,'Cold dishes',2)").await;

    let page = Category::select_page(&rb, &page_request(), &CategoryPageDTO { category_type: Some(1), name: Some("Hot".into()) }).await?;
    assert_eq!(page.total, 1);
    assert_eq!(page.records[0].name.as_deref(), Some("Hot dishes"));
    assert_eq!(page.records[0].category_type, Some(1));
    Ok(())
}

#[tokio::test]
async fn dish_html_filters_page_and_flavor_join() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO dish (id,name,category_id,price,status,sort,is_deleted) VALUES ('d-1','Kung Pao','c-1','20.00',1,1,0),('d-2','Soup','c-2','10.00',0,2,0)").await;
    insert(&rb, "INSERT INTO dish_flavor (id,dish_id,name,value,is_deleted) VALUES ('f-1','d-1','spice','medium',0)").await;

    let page = Dish::select_page(&rb, &page_request(), &DishPageDTO { name: Some("Kung".into()), category_id: Some("c-1".into()), status: Some(1) }).await?;
    assert_eq!(page.total, 1);
    let rows = Dish::select_dish_with_flavors(&rb, Some("d-1"), None).await?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].flavor_name.as_deref(), Some("spice"));
    Ok(())
}

#[tokio::test]
async fn dish_flavor_html_filters_active_flavors() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO dish_flavor (id,dish_id,name,value,is_deleted) VALUES ('f-1','d-1','spice','hot',0),('f-2','d-1','spice','old',1)").await;

    let page = DishFlavor::select_page(&rb, &page_request(), &DishFlavorPageDTO { dish_id: Some("d-1".into()), name: Some("spice".into()) }).await?;
    assert_eq!(page.total, 1);
    assert_eq!(page.records[0].value.as_deref(), Some("hot"));
    Ok(())
}

#[tokio::test]
async fn employee_html_filters_multiple_conditions() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO employee (id,name,username,phone,status) VALUES ('e-1','Alice','alice','13800000000',1),('e-2','Bob','bob','13900000000',0)").await;

    let page = Employee::select_page(&rb, &page_request(), &EmployeePageDTO { name: Some("Ali".into()), username: Some("ali".into()), phone: Some("138".into()), status: Some(1) }).await?;
    assert_eq!(page.total, 1);
    assert_eq!(page.records[0].username.as_deref(), Some("alice"));
    Ok(())
}

#[tokio::test]
async fn order_detail_html_filters_order_and_dish() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO order_detail (id,name,order_id,dish_id,number,amount) VALUES ('od-1','Kung Pao','o-1','d-1',1,'20.00'),('od-2','Soup','o-2','d-2',1,'10.00')").await;

    let page = OrderDetail::select_page(&rb, &page_request(), &OrderDetailPageDTO { order_id: Some("o-1".into()), dish_id: Some("d-1".into()), setmeal_id: None }).await?;
    assert_eq!(page.total, 1);
    assert_eq!(page.records[0].name.as_deref(), Some("Kung Pao"));
    Ok(())
}

#[tokio::test]
async fn orders_html_filters_page_and_details_join() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO orders (id,number,status,user_id,pay_status,amount) VALUES ('o-1','ORD-1',1,'u-1',1,'20.00'),('o-2','ORD-2',0,'u-2',0,'10.00')").await;
    insert(&rb, "INSERT INTO order_detail (id,name,order_id,dish_id,number,amount) VALUES ('od-1','Kung Pao','o-1','d-1',1,'20.00')").await;

    let page = Orders::select_page(&rb, &page_request(), &OrdersPageDTO { number: Some("ORD-1".into()), status: Some(1), user_id: Some("u-1".into()), address_book_id: None }).await?;
    assert_eq!(page.total, 1);
    let rows = Orders::select_order_with_details(&rb, Some("o-1"), None).await?;
    assert_eq!(rows[0].detail_id.as_deref(), Some("od-1"));
    Ok(())
}

#[tokio::test]
async fn setmeal_html_filters_active_and_dish_join() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO setmeal (id,category_id,name,price,status,is_deleted) VALUES ('s-1','c-1','Lunch set','30.00',1,0),('s-2','c-2','Old set','20.00',0,1)").await;
    insert(&rb, "INSERT INTO setmeal_dish (id,setmeal_id,dish_id,name,price,copies,sort) VALUES ('sd-1','s-1','d-1','Kung Pao','20.00',1,1)").await;

    let page = Setmeal::select_page(&rb, &page_request(), &SetmealPageDTO { name: Some("Lunch".into()), category_id: Some("c-1".into()), status: Some(1) }).await?;
    assert_eq!(page.total, 1);
    let rows = Setmeal::select_setmeal_with_dishes(&rb, Some("s-1"), None).await?;
    assert_eq!(rows[0].dish_name.as_deref(), Some("Kung Pao"));
    Ok(())
}

#[tokio::test]
async fn setmeal_dish_html_filters_relation() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO setmeal_dish (id,setmeal_id,dish_id,name,price,copies,sort) VALUES ('sd-1','s-1','d-1','Kung Pao','20.00',1,1),('sd-2','s-2','d-2','Soup','10.00',1,2)").await;

    let page = SetmealDish::select_page(&rb, &page_request(), &SetmealDishPageDTO { setmeal_id: Some("s-1".into()), dish_id: Some("d-1".into()) }).await?;
    assert_eq!(page.total, 1);
    assert_eq!(page.records[0].id.as_deref(), Some("sd-1"));
    Ok(())
}

#[tokio::test]
async fn shopping_cart_html_filters_item_owner() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO shopping_cart (id,user_id,dish_id,number,amount) VALUES ('sc-1','u-1','d-1',2,'40.00'),('sc-2','u-2','d-1',1,'20.00')").await;

    let page = ShoppingCart::select_page(&rb, &page_request(), &ShoppingCartPageDTO { user_id: Some("u-1".into()), dish_id: Some("d-1".into()), setmeal_id: None }).await?;
    assert_eq!(page.total, 1);
    assert_eq!(page.records[0].id.as_deref(), Some("sc-1"));
    Ok(())
}

#[tokio::test]
async fn sys_dict_html_filters_code_and_name() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO sys_dict (id,name,code,state) VALUES ('dict-1','Enabled','feature_enabled',1),('dict-2','Disabled','feature_disabled',0)").await;

    let page = SysDict::select_page(&rb, &page_request(), &DictPageDTO { code: Some("enabled".into()), name: Some("Enabled".into()) }).await?;
    assert_eq!(page.total, 1);
    assert_eq!(page.records[0].id.as_deref(), Some("dict-1"));
    Ok(())
}

#[tokio::test]
async fn user_html_filters_name_phone_and_status() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO \"user\" (id,name,phone,status) VALUES ('u-1','Alice','13800000000',1),('u-2','Bob','13900000000',0)").await;

    let page = User::select_page(&rb, &page_request(), &UserPageDTO { name: Some("Ali".into()), phone: Some("138".into()), status: Some(1) }).await?;
    assert_eq!(page.total, 1);
    assert_eq!(page.records[0].id.as_deref(), Some("u-1"));
    Ok(())
}

#[tokio::test]
async fn paginate_html_applies_limit_offset_and_count() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    rb.exec("CREATE TABLE pager_item (id INTEGER)", vec![]).await?;
    insert(&rb, "INSERT INTO pager_item (id) VALUES (1),(2),(3)").await;

    let rows = PaginateMapper::paginate_query(&rb, "SELECT id FROM pager_item ORDER BY id", 1, 1).await?;
    assert_eq!(rows.len(), 1);
    let total = PaginateMapper::count_query(&rb, "SELECT id FROM pager_item").await?;
    assert_eq!(total, 3);
    Ok(())
}

#[tokio::test]
async fn join_queries_html_executes_all_six_reports() -> Result<(), Box<dyn Error>> {
    let rb = test_db().await;
    insert(&rb, "INSERT INTO category (id,type,name,sort) VALUES ('c-1',1,'Main course',1)").await;
    insert(&rb, "INSERT INTO dish (id,name,category_id,price,code,image,description,status,sort,is_deleted) VALUES ('d-1','Kung Pao','c-1','20.00','D-1','dish.jpg','spicy',1,1,0)").await;
    insert(&rb, "INSERT INTO dish_flavor (id,dish_id,name,value,is_deleted) VALUES ('f-1','d-1','spice','hot',0)").await;
    insert(&rb, "INSERT INTO setmeal (id,category_id,name,price,status,code,description,image,is_deleted) VALUES ('s-1','c-1','Lunch set','30.00',1,'S-1','daily','set.jpg',0)").await;
    insert(&rb, "INSERT INTO setmeal_dish (id,setmeal_id,dish_id,name,price,copies,sort) VALUES ('sd-1','s-1','d-1','Kung Pao','20.00',1,1)").await;
    insert(&rb, "INSERT INTO \"user\" (id,openid,name,phone,sex,status) VALUES ('u-1','open-1','Alice','13800000000',1,1)").await;
    insert(&rb, "INSERT INTO address_book (id,user_id,consignee,phone,label,is_default,is_deleted) VALUES ('ab-1','u-1','Alice','13800000000','home',1,0)").await;
    insert(&rb, "INSERT INTO address_book (id,user_id,consignee,phone,label,is_default,is_deleted) VALUES ('ab-2','u-1','Alice','13800000000','work',1,0)").await;
    insert(&rb, "INSERT INTO orders (id,number,status,user_id,order_time,pay_method,pay_status,amount,consignee,phone,address) VALUES ('o-1','ORD-1',1,'u-1','2026-08-15 12:00:00',1,1,'20.00','Alice','13800000000','Road 1')").await;
    insert(&rb, "INSERT INTO order_detail (id,name,order_id,dish_id,number,amount) VALUES ('od-1','Kung Pao','o-1','d-1',1,'20.00')").await;
    insert(&rb, "INSERT INTO shopping_cart (id,user_id,dish_id,number,amount,create_time) VALUES ('sc-1','u-1','d-1',1,'20.00','2026-08-15 12:00:00')").await;
    insert(&rb, "INSERT INTO shopping_cart (id,user_id,dish_id,number,amount,create_time) VALUES ('sc-2','u-1','d-1',1,'20.00','2026-08-15 12:00:01')").await;

    let orders = JoinQuery::select_order_with_details(&rb, Some("o-1"), Some("u-1"), Some(1)).await?;
    assert_eq!(orders[0].dish_name.as_deref(), Some("Kung Pao"));
    let setmeals = JoinQuery::select_setmeal_with_dishes(&rb, Some("s-1"), Some("c-1"), Some(1)).await?;
    assert_eq!(setmeals[0].dish_id.as_deref(), Some("d-1"));
    let carts = JoinQuery::select_shopping_cart_with_items(&rb, Some("u-1"), Some("sc-1")).await?;
    assert_eq!(carts.len(), 1);
    assert_eq!(carts[0].item_type.as_deref(), Some("dish"));
    let dishes = JoinQuery::select_dish_full_info(&rb, Some("d-1"), Some("c-1"), Some(1)).await?;
    assert!(dishes[0].flavors.as_deref().is_some_and(|flavors| flavors.contains("spice:hot")));
    let users = JoinQuery::select_user_with_order_stats(&rb, Some("u-1"), Some("13800000000")).await?;
    assert_eq!(users[0].order_count, Some(1));
    let addresses = JoinQuery::select_address_book_with_user(&rb, Some("u-1"), Some("ab-1"), Some(1)).await?;
    assert_eq!(addresses.len(), 1);
    assert_eq!(addresses[0].address_id.as_deref(), Some("ab-1"));
    assert_eq!(addresses[0].user_name.as_deref(), Some("Alice"));
    Ok(())
}
