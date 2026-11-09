use rand::Rng;
use rbatis::crud;
use rbatis::rbatis::RBatis;
use rbatis::rbdc::DateTime;
use rbdc_sqlite::driver::SqliteDriver;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::fs;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    fs::create_dir_all("logs").expect("create logs directory");
    let log_file = std::fs::File::create("logs/rbatis.log").expect("create rbatis log file");
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        log_file,
    )])
    .expect("initialize combined logger");

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
        name: Some("测试字典".to_string()),
        code: Some("test_dict".to_string()),
        state: Some(1),
        create_date: Some(DateTime::now()),
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

    // 测试分页查询
    println!("\n=== 测试分页查询 ===");
    let page = 1;
    let page_size = 10;
    let offset = (page - 1) * page_size;

    let sql = format!(
        "SELECT * FROM sys_dict LIMIT {} OFFSET {}",
        page_size, offset
    );
    let result = rb.query_decode::<Vec<SysDict>>(&sql, vec![]).await;
    match result {
        Ok(dicts) => {
            println!("查询到 {} 条数据", dicts.len());
            for dict in dicts {
                println!("{:?}", dict);
            }
        }
        Err(e) => println!("分页查询失败: {:?}", e),
    }

    // 测试通用分页查询
    println!("\n=== 测试通用分页查询 ===");
    let sql = "SELECT * FROM sys_dict";
    let count_sql = "SELECT COUNT(*) FROM sys_dict";

    let data = rb.query_decode::<Vec<SysDict>>(sql, vec![]).await?;
    let _count_result = rb
        .query_decode::<Vec<serde_json::Value>>(count_sql, vec![])
        .await?;

    println!("查询到 {} 条数据", data.len());
    for item in data {
        println!("{:?}", item);
    }

    Ok(())
}
