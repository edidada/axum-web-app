use std::fmt::Write;
use std::thread;
use tracing::{span, Level, info};
use tracing_subscriber::{
    fmt::{
        format::{Format, Writer},
        FmtContext, FormatEvent, FormatFields, time::FormatTime,
    },
    layer::Context,
    registry::LookupSpan,
};
use chrono::Local;
use tracing_appender::non_blocking;

// 自定义时间格式化结构体
struct LocalTimer;

// 实现 FormatTime trait 以自定义时间格式
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
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

use tracing_subscriber::{fmt, layer::SubscriberExt, Registry};
use std::net::SocketAddr;

fn setup_logging() {
    // 创建一个滚动文件追加器，这里简单示例为非滚动的文件写入
    let file_appender = std::fs::File::create("app.log").expect("Failed to create log file");
    let (non_blocking, _guard) = non_blocking(file_appender);

    // 创建一个日志记录器层，将日志输出到文件
    let file_layer = fmt::layer()
        .event_format(ThreadIdFormatter)
        .with_timer(LocalTimer)
        .with_ansi(false) // 禁用 ANSI 样式
        .with_writer(non_blocking);

    // 创建一个格式化层，将日志输出到控制台
    let console_layer = fmt::layer()
        .event_format(ThreadIdFormatter)
        .with_timer(LocalTimer)
        .pretty();

    // 创建一个订阅者，将文件层和控制台层添加到订阅者中
    let subscriber = Registry::default()
        .with(console_layer)
        .with(file_layer);

    // 设置全局订阅者
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}

fn main() {
    setup_logging();

    let payload = "example_payload";
    info!("get_body_handler {:?}", payload);

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    info!("Listening on {}", addr);
}