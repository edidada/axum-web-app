use rbatis::rbatis::RBatis;
use rbdc_sqlite::driver::SqliteDriver;
use rbatis::{crud, html_sql};
use rbatis::rbdc::DateTime;
use rbatis::PageRequest;

///dictionary table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

crud!(SysDict {});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DictPageDTO {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[html_sql("src/domain/table/sys_dict.html")]
impl SysDict {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &DictPageDTO) -> rbatis::Result<rbatis::Page<SysDict>> { impled!() }
}

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
    let dict1 = SysDict {
        id: Some("1".to_string()),
        name: Some("测试字典1".to_string()),
        code: Some("test_dict1".to_string()),
        state: Some(1),
        create_date: Some(DateTime::now()),
    };
    
    let dict2 = SysDict {
        id: Some("2".to_string()),
        name: Some("测试字典2".to_string()),
        code: Some("test_dict2".to_string()),
        state: Some(1),
        create_date: Some(DateTime::now()),
    };
    
    let result1 = SysDict::insert(&rb, &dict1).await;
    match result1 {
        Ok(_) => println!("增加数据1成功"),
        Err(e) => println!("增加数据1失败: {:?}", e),
    }
    
    let result2 = SysDict::insert(&rb, &dict2).await;
    match result2 {
        Ok(_) => println!("增加数据2成功"),
        Err(e) => println!("增加数据2失败: {:?}", e),
    }
    
    // 测试查询数据
    println!("\n=== 测试查询数据 ===");
    let result = SysDict::select_all(&rb).await;
    match result {
        Ok(dicts) => {
            println!("查询到 {} 条数据", dicts.len());
            for dict in dicts {
                println!("{:?}", dict);
            }
        }
        Err(e) => println!("查询数据失败: {:?}", e),
    }
    
    // 测试使用 HTML SQL 模板的分页查询
    println!("\n=== 测试使用 HTML SQL 模板的分页查询 ===");
    let page_request = PageRequest::new(1, 10);
    let dto = DictPageDTO {
        name: None,
        code: None,
    };
    
    let result = SysDict::select_page(&rb, &page_request, &dto).await;
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
    
    // 测试带条件的分页查询
    println!("\n=== 测试带条件的分页查询 ===");
    let dto_with_condition = DictPageDTO {
        name: Some("测试字典1".to_string()),
        code: None,
    };
    
    let result = SysDict::select_page(&rb, &page_request, &dto_with_condition).await;
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
        Err(e) => println!("带条件的分页查询失败: {:?}", e),
    }
    
    Ok(())
}