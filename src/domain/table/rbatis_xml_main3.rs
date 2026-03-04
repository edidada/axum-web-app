use rbatis::rbatis::RBatis;
use rbdc_sqlite::driver::SqliteDriver;
use rbatis::PageRequest;

mod sys_dict;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 RBatis
    let rb = RBatis::new();
    rb.init(SqliteDriver {}, "sqlite://./test.db")?;
    
    // 创建表
    println!("=== 创建表 ===");
    let create_table_sql = r#"
    CREATE TABLE IF NOT EXISTS sys_dict (
        id TEXT PRIMARY KEY,
        name TEXT,
        code TEXT,
        state INTEGER,
        create_date TEXT
    )
    "#;
    rb.exec(create_table_sql, vec![]).await?;
    println!("表创建成功");
    
    // 测试增加数据
    println!("\n=== 测试增加数据 ===");
    let dict = sys_dict::SysDict {
        id: Some("5".to_string()),
        name: Some("测试字典5".to_string()),
        code: Some("test_dict5".to_string()),
        state: Some(1),
        create_date: Some(rbatis::rbdc::DateTime::now()),
    };
    
    let result = sys_dict::SysDict::insert(&rb, &dict).await;
    match result {
        Ok(_) => println!("增加数据成功"),
        Err(e) => println!("增加数据失败: {:?}", e),
    }
    
    // 测试查询数据
    println!("\n=== 测试查询数据 ===");
    let result = sys_dict::SysDict::select_all(&rb).await;
    match result {
        Ok(dicts) => {
            println!("查询到 {} 条数据", dicts.len());
            for dict in dicts {
                println!("{:?}", dict);
            }
        }
        Err(e) => println!("查询数据失败: {:?}", e),
    }
    
    // 测试使用 select_page API
    println!("\n=== 测试使用 select_page API ===");
    let page_request = PageRequest::new(1, 10);
    let dto = sys_dict::DictPageDTO {
        name: None,
        code: None,
    };
    
    let result = sys_dict::SysDict::select_page(&rb, &page_request, &dto).await;
    match result {
        Ok(page) => {
            println!("总记录数: {}", page.total);
            println!("当前页码: {}", page.page_no);
            println!("每页大小: {}", page.page_size);
            println!("查询到 {} 条数据", page.records.len());
            for dict in page.records {
                println!("{:?}", dict);
            }
        }
        Err(e) => println!("分页查询失败: {:?}", e),
    }
    
    Ok(())
}