use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use chrono::Local;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::format::Writer;

fn main() {
    // 创建一个按天滚动的日志文件追加器
    let appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log")
        .max_log_file_size(0.5 * 1024 * 1024); // 设置文件最大大小为 0.5MB
    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

    // 创建一个日志记录器层，将日志输出到文件
    let file_layer = fmt::layer()
        .with_timer(LocalTimer)
        .with_writer(non_blocking);

    // 创建一个基于环境变量的过滤器层
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 创建一个订阅器，将过滤器层和文件层组合起来
    let subscriber = Registry::default()
        .with(env_filter)
        .with(file_layer);

    // 设置全局订阅器
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let addr = "127.0.0.1:8080";
    tracing::info!("Listening on {}", addr);
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}