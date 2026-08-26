use log::LevelFilter;
use rand::Rng;
use rbatis::rbatis::RBatis;
use rbatis::rbdc::DateTime;
use rbatis::PageRequest;
use rbatis::{crud, html_sql};
use rbdc_sqlite::driver::SqliteDriver;
use simplelog::{CombinedLogger, Config, WriteLogger};
use std::fs;

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
    pub async fn select_page(
        conn: &dyn rbatis::Executor,
        page_request: &rbatis::PageRequest,
        dto: &DictPageDTO,
    ) -> rbatis::Result<rbatis::Page<SysDict>> {
        impled!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    fs::create_dir_all("logs").unwrap();
    let log_file = std::fs::File::create("logs/rbatis.log").unwrap();
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        log_file,
    )])
    .unwrap();
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
    let mut rng = rand::thread_rng();
    let random_id = rng.gen::<u64>();

    let dict = SysDict {
        id: Some(random_id.to_string()),
        name: Some("测试字典3".to_string()),
        code: Some("test_dict3".to_string()),
        state: Some(1),
        create_date: Some(rbatis::rbdc::DateTime::now()),
    };

    let result = SysDict::insert(&rb, &dict).await;
    match result {
        Ok(_) => println!("增加数据成功"),
        Err(e) => println!("增加数据失败: {:?}", e),
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

    // 测试使用 select_page API
    println!("\n=== 测试使用 select_page API ===");
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

    // 测试使用其他 CRUD API
    println!("\n=== 测试使用其他 CRUD API ===");

    // 测试查询所有数据
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

    Ok(())
}
