use std::error::Error;
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::thread;
use chrono::Local;
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::fmt::format::{Format, Writer};
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use redis::Client;
use tower::ServiceBuilder;
use tracing_appender::non_blocking::{WorkerGuard};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use crate::config::Config;
use rbatis::rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use rbs::Value;

mod handlers;
mod routes;
mod middleware;
mod config;
mod domain;

// 定义静态的 RBatis 实例
static mut RB: Option<RBatis> = None;

// 异步初始化函数
async fn init_rbatis() -> Result<(), Box<dyn Error>> {
    let rb = RBatis::new();
    // rbatis 默认已经启用了 SQL 日志，日志级别为 Debug
    rb.init(MysqlDriver {}, "mysql://github:5%25Edidadas@114.55.62.154:3306/testdb")?;
    unsafe {
        RB = Some(rb);
    }
    Ok(())
}

// 获取单例的函数
fn get_rbatis() -> &'static RBatis {
    unsafe {
        RB.as_ref().expect("RBatis is not initialized")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化 RBatis
    init_rbatis().await?;

    let _guard = set_format_log();

    let config_file = Config::from_file();
    // Initialize Redis client
    let redis_client = Client::open(config_file.webapp.redisurl).unwrap_or_else(|e| {
        eprintln!("Warning: Failed to connect to Redis: {}. Using fallback.", e);
        // 使用本地 Redis 作为 fallback
        Client::open("redis://127.0.0.1:6379").unwrap()
    });

    // 获取 RBatis 实例
    let rb = get_rbatis();

    let app = Router::new()
        .merge(routes::setup_routes())
        .layer(
            ServiceBuilder::new()
                .layer(Extension(redis_client))
                .layer(Extension(rb))
        );

    // Change the port number here if necessary
    let addr = SocketAddr::from(([127, 0, 0, 1], config_file.webapp.port));
    tracing::info!("thread {:?},Listening on {}", thread::current().id(), addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn set_format_log() -> WorkerGuard {
    // 初始化 log 到 tracing 的桥接
    tracing_log::LogTracer::init().expect("Failed to init LogTracer");
    
    // 创建一个滚动文件追加器
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 创建一个日志记录器层，将日志输出到文件
    let file_layer = fmt::layer()
        .with_timer(LocalTimer {}) // 传递 LocalTimer 实例
        .event_format(ThreadIdFormatter)
        .with_ansi(false) // 禁用 ANSI 样式
        .with_writer(non_blocking);

    // 创建一个格式化层，将日志输出到控制台
    let console_layer = fmt::layer()
        .with_timer(LocalTimer {}) // 传递 LocalTimer 实例
        .event_format(ThreadIdFormatter);

    // 创建一个订阅者，将文件层和控制台层添加到订阅者中
    let subscriber = Registry::default()
        .with(console_layer)
        .with(file_layer);

    // 设置全局订阅者
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    guard
}

// 自定义日志格式化结构体
struct ThreadIdFormatter;

impl<S, N> FormatEvent<S, N> for ThreadIdFormatter
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        // 获取当前线程 ID
        let thread_id = thread::current().id();
        // 写入线程 ID 信息
        write!(writer, "thread {:?}, ", thread_id)?;

        // 调用默认的格式化方法处理事件
        <Format as FormatEvent<S, N>>::format_event(&Format::default(), ctx, writer, event)
    }
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}


#[cfg(test)]
mod tests {
    use rbatis::{PageRequest, RBatis};
    use rbdc_sqlite::SqliteDriver;
    use crate::domain::table::sys_dict;
    use tracing_subscriber::{fmt, EnvFilter, Registry, layer::SubscriberExt};
    use tracing_appender::rolling::{RollingFileAppender, Rotation};

    #[tokio::test]
    async fn test_sys_dict() -> Result<(), Box<dyn std::error::Error>> {

        let appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

        // 创建一个日志记录器层，将日志输出到文件
        let file_layer = fmt::layer()
            .with_ansi(false) // 禁用 ANSI 样式
            .with_writer(non_blocking);

        // 创建一个基于环境变量的过滤器层
        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        // 创建一个订阅器，将过滤器层和文件层组合起来
        let subscriber = Registry::default()
            .with(env_filter)
            .with(file_layer);

        // 设置全局订阅器
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

        // 初始化 RBatis
        let rb = RBatis::new();
        rb.init(SqliteDriver {}, "sqlite://./test.db")?;

        // 创建表
        tracing::info!("=== 创建表 ===");
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
        tracing::info!("=== 表创建成功 ===");

        // 测试增加数据
        tracing::info!("=== 测试增加数据 ===");
        let dict = sys_dict::SysDict {
            id: Some("5".to_string()),
            name: Some("测试字典5".to_string()),
            code: Some("test_dict5".to_string()),
            state: Some(1),
            create_date: Some(rbatis::rbdc::DateTime::now()),
        };

        let result = sys_dict::SysDict::insert(&rb, &dict).await;
        match result {
            Ok(_) => tracing::info!("增加数据成功"),
            Err(e) => tracing::info!("增加数据失败: {:?}", e),
        }

        // 测试查询数据
        tracing::info!("\n=== 测试查询数据 ===");
        let result = sys_dict::SysDict::select_all(&rb).await;
        match result {
            Ok(dicts) => {
                tracing::info!("查询到 {} 条数据", dicts.len());
                for dict in dicts {
                    tracing::info!("{:?}", dict);
                }
            }
            Err(e) => tracing::info!("查询数据失败: {:?}", e),
        }

        // 测试使用 select_page API
        tracing::info!("\n=== 测试使用 select_page API ===");
        let page_request = PageRequest::new(1, 10);
        let dto = crate::domain::dto::dict_dto::DictPageDTO {
            name: None,
            code: None,
        };

        let result = sys_dict::SysDict::select_page(&rb, &page_request, &dto).await;
        match result {
            Ok(page) => {
                tracing::info!("总记录数: {}", page.total);
                tracing::info!("当前页码: {}", page.page_no);
                tracing::info!("每页大小: {}", page.page_size);
                tracing::info!("查询到 {} 条数据", page.records.len());
                for dict in page.records {
                    tracing::info!("{:?}", dict);
                }
            }
            Err(e) => tracing::info!("分页查询失败: {:?}", e),
        }

        Ok(())
    }
}
