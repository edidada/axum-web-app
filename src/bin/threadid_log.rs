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
    Layer,
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

// 自定义事件格式化器
struct ThreadIdFormatter;

impl<S, N> FormatEvent<S, N> for ThreadIdFormatter
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();
        let span = ctx.lookup_current().map(|s| s.name()).unwrap_or("unknown");
        let thread_id = thread::current().id();
        write!(
            writer,
            "[{} {:?} {}:{}] ",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            thread_id,
            span,
            meta.target()
        )?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

fn main() {
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_timer(LocalTimer) // Move this line before event_format
        .event_format(ThreadIdFormatter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let span = span!(Level::INFO, "main");
    let _enter = span.enter();

    info!("This is an info message with thread ID and custom time format.");
}