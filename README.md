# Axum Web Application

This is a simple web application built using the Axum framework in Rust. The application demonstrates how to set up a basic web server, handle requests, and define routes.

## Project Structure

```
axum-web-app
├── src
│   ├── main.rs          # Entry point of the application
│   ├── handlers         # Contains request handlers
│   │   └── mod.rs
│   ├── routes           # Defines application routes
│   │   └── mod.rs
│   └── models           # Data models used in the application
│       └── mod.rs
├── Cargo.toml           # Cargo configuration file
└── README.md            # Project documentation
```

## Setup Instructions

1. Ensure you have Rust and Cargo installed on your machine. You can install them from [rustup.rs](https://rustup.rs/).

2. Clone the repository:

   ```
   git clone <repository-url>
   cd axum-web-app
   ```

3. Build the project:

   ```
   cargo build
   ```

4. Run the application:

   ```
   cargo run
   ```

## Usage

Once the application is running, you can access it at `http://localhost:3000`. You can test the endpoints using tools like `curl` or Postman.

### Example Endpoints

- GET /users: Retrieve a list of users.
- POST /users: Create a new user.

## Contributing

Feel free to submit issues or pull requests if you have suggestions or improvements for the project.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Mac

```shell
cargo run
error: package `mio v1.0.3` cannot be built because it requires rustc 1.70 or newer, while the currently active rustc version is 1.65.0
Either upgrade to rustc 1.70 or newer, or use
cargo update -p mio@1.0.3 --precise ver
where `ver` is the latest version of `mio` supporting rustc 1.65.0
```

127.0.0.1:9001/get
127.0.0.1:9001/post
curl -X POST 'http://127.0.0.1:9001/getBody' -H 'Content-Type: application/json' -d '{
"html_code": "a"
}'

cargo run --color=always --package axum-web-app --bin main --profile dev

C:/Users/edida/.cargo/bin/cargo.exe run --color=always --package axum-web-app --bin axum-web-app --profile dev

powershell
$env:RUST_BACKTRACE = "full"


    Finished `dev` profile [unoptimized + debuginfo] target(s) in 16.69s
     Running `target\debug\axum-web-app.exe`
thread 'main' panicked at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tracing-subscriber-0.3.19\src\fmt\mod.rs:517:14:
Unable to install global subscriber: SetGlobalDefaultError("a global default trace dispatcher has already been set")
stack backtrace:
   0:     0x7ff69dc466ad - std::backtrace_rs::backtrace::dbghelp64::trace
                               at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library\std\src\..\..\backtrace\src\backtrace\dbghelp64.rs:91
   1:     0x7ff69dc466ad - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff69dc466ad - std::sys_common::backtrace::_print_fmt
                               at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library\std\src\sys_common\backtrace.rs:68


C:/Users/edida/.cargo/bin/cargo.exe build --color=always --message-format=json-diagnostic-rendered-ansi --package axum-web-app --bin axum-web-app --profile dev

C:/Users/edida/.cargo/bin/cargo.exe build --color=always --message-format=json-diagnostic-rendered-ansi --package axum-web-app --bin log_chinese --profile dev
C:/Users/edida/.cargo/bin/cargo.exe build --message-format=json-diagnostic-rendered-ansi --color=always --package axum-web-app --bin main --profile dev

cargo run --package axum-web-app --bin main --profile dev

cargo test -- --test-threads=1


你看到的像 `[2m`、`[0m`、`[32m` 这样的字符序列其实是 ANSI 转义序列（ANSI escape sequences），它们用于在终端中实现文本样式（如颜色、加粗、下划线等）的控制。当这些序列没有被正确处理时，就会以原始的形式显示出来，导致日志中出现这些看起来奇怪的字符。

### 问题原因
通常，这些转义序列会被终端解释并应用相应的样式，但如果日志输出到不支持 ANSI 转义序列的环境（比如某些日志文件查看器、不支持 ANSI 样式的 IDE 控制台等），就会原样显示。

### 解决办法

#### 1. 禁用 ANSI 转义序列
如果你不需要日志中的样式，可以在日志配置中禁用 ANSI 转义序列的使用。以 `tracing` 和 `tracing-subscriber` 为例，以下是一个示例代码：

```rust
use tracing::{info, Level};
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // 配置日志，禁用 ANSI 样式
    let subscriber = fmt::Subscriber::builder()
       .with_env_filter(EnvFilter::from_default_env())
       .with_ansi(false) // 禁用 ANSI 样式
       .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let html_code = "sssa";
    info!("get_body_handler RequestBody {{ html_code: \"{}\" }}", html_code);
}
```

在上述代码中，`with_ansi(false)` 方法被用来禁用 ANSI 转义序列的输出。这样，日志就不会包含那些用于样式控制的特殊字符了。

#### 2. 使用支持 ANSI 转义序列的工具查看日志
如果你希望保留日志中的样式，可以使用支持 ANSI 转义序列的工具来查看日志。例如，在 Linux 或 macOS 系统中，可以使用 `less -R` 命令来查看包含 ANSI 样式的日志文件：

```bash
less -R your_log_file.log
```

在 Windows 系统中，一些终端模拟器（如 Windows Terminal）也支持 ANSI 转义序列，你可以在这些终端中查看日志以正确显示样式。

如果你想要在配置日志记录器时添加 `.with_ansi(false)` 来禁用 ANSI 样式，并且结合你提供的 `FormatTime` 实现，下面是一个完整的示例，展示了如何使用 `tracing` 和 `tracing-subscriber` 来完成这件事。

### 完整代码示例
```rust
use chrono::{Local, TimeZone};
use tracing::{info, Level};
use tracing_subscriber::{fmt, EnvFilter, fmt::time::FormatTime, fmt::Writer};

// 自定义时间格式化结构体
struct LocalTimer;

// 实现 FormatTime trait 以自定义时间格式
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

fn main() {
    // 配置日志记录器
    let subscriber = fmt::Subscriber::builder()
       .with_env_filter(EnvFilter::from_default_env())
       .with_timer(LocalTimer) // 使用自定义的时间格式化器
       .with_ansi(false) // 禁用 ANSI 样式
       .finish();

    // 设置全局日志订阅者
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // 记录一条信息日志
    info!("This is an info log message.");
}
```

### 代码解释
1. `LocalTimer` 结构体和 `FormatTime` 实现：
    - 定义了 `LocalTimer` 结构体，用于自定义时间格式化。
    - 实现了 `FormatTime` trait 的 `format_time` 方法，该方法使用 `chrono` 库将当前本地时间格式化为 `%Y-%m-%d %H:%M:%S` 的形式，并写入到 `Writer` 中。

2. 日志记录器配置：
    - 使用 `fmt::Subscriber::builder()` 构建一个日志订阅者。
    - `with_env_filter(EnvFilter::from_default_env())`：根据环境变量配置日志过滤规则。
    - `with_timer(LocalTimer)`：使用自定义的 `LocalTimer` 作为时间格式化器。
    - `with_ansi(false)`：禁用 ANSI 转义序列，这样日志输出就不会包含用于样式控制的特殊字符。
    - `finish()`：完成订阅者的构建。

3. 设置全局日志订阅者：
    - 使用 `tracing::subscriber::set_global_default` 方法将构建好的订阅者设置为全局日志订阅者。

4. 记录日志：
    - 使用 `info!` 宏记录一条信息级别的日志。

### 依赖添加
确保在 `Cargo.toml` 文件中添加必要的依赖：
```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
chrono = "0.4"
```

这样，日志输出就不会包含 ANSI 转义序列，并且会使用你自定义的时间格式。 
