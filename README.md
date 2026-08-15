# Axum Web Application

frontend是web ui代码仓库
spider是selenium自动化测试文件夹
自动测试web，selenium工具，新建单独文件夹，测试登录功能
pg database用系统安装好的，只保留新建表的sql，处理数据的sql
文件放在db文件夹

cargo build --release

在Rust中，有几种通过命令行工具“一键”移除未使用导入（unused import）警告的方法。

这里推荐两种主流且高效的工具：
方法一：使用 rust-analyzer 的 fix 功能 (最推荐)
rust-analyzer 是Rust官方的LSP（语言服务器），其 fix 命令可以直接在命令行中应用代码修复。
1.  首先，确保已安装 rust-analyzer：
    # 通常通过 rustup 安装
    rustup component add rust-analyzer
2.  进入你的项目根目录，然后执行：
    # 对当前目录下的所有Rust文件，应用所有可用的自动修复（包括未使用的导入）
    cargo fix
    这是最推荐的方法。cargo fix 是官方工具链的一部分，它会自动运行 cargo check，并尝试应用Rust编译器建议的所有“可自动应用的”修复。这能处理大部分未使用的导入警告，以及其他类似的问题。
    ◦   修复范围：默认修复当前包。使用 --workspace 标志可修复整个工作空间。
    ◦   更精确地修复未使用导入：虽然 cargo fix 是综合性的，但你可以通过指定 lint 来专门处理未使用的导入：
        cargo fix --allow-staged -- -W unused-imports

方法二：使用 rustfmt 配合 rustfix 工具
rustfmt 是Rust的官方代码格式化工具，但它本身不负责修复未使用的导入。更纯粹的工具链组合是：
1.  使用 cargo clippy 的 --fix 选项（Clippy是更严格的代码检查器）：
    cargo clippy --fix --allow-staged
    执行后，Clippy会输出它发现的可自动修复的问题（包括未使用的导入），并询问你是否应用这些修复。输入 y 确认即可。
2.  专门针对未使用导入运行Clippy修复：
    cargo clippy --fix --allow-staged -- -W clippy::single_component_path_imports
    这条命令会更精确地针对“未使用的单组件导入路径”这类警告进行修复。
方法三：使用IDE/编辑器的“快速修复”功能 (非命令行，但效率最高)
这虽然不是纯命令行操作，但在日常开发中通常是最快的“一键”方式。
•   VS Code + rust-analyzer：
    将光标放在波浪线警告上，或打开有问题的文件，按 Ctrl + . (Windows/Linux) 或 Cmd + . (macOS)，然后选择 “Remove unused import” 或 “Fix all unused imports in file”。
•   IntelliJ IDEA / CLion 的 Rust 插件：
    同样，将光标放在警告上，按 Alt + Enter，然后选择对应的修复选项。
总结与最佳实践
方法 命令 特点
推荐：官方一键修复 cargo fix 最简单、最安全，是Rust官方工具链的一部分，可修复编译器能自动修复的所有问题。
推荐：Clippy增强修复 cargo clippy --fix 可检测并修复更多代码风格和潜在问题，但可能包含更主观的修改。
编辑器辅助 Ctrl + . 或 Alt + Enter 交互式，可逐条或按文件批量修复，适合集成到开发流程中。

核心建议：
对于命令行下的“一键”清理，进入项目根目录后，直接运行：
cargo fix

如果还有一些Clippy特有的警告，可以再运行：
cargo clippy --fix --allow-staged
这将为你清理绝大多数由未使用导入、未使用变量等引起的编译警告。

```shell
No function or associated item delete_by_id found in the current scope for struct SysDict [E0599] 
// 假设你已经拿到了要删除的 id（String 类型） let id_to_delete = random_id.to_string(); 
// 或从请求中获取 let delete_result = sys_dict::SysDict::delete_by_id(&rb, &id_to_delete).await?; 
tracing::info!("按 id 删除结果: affected rows = {}", delete_result.rows_affected);
```
原因：在你当前的rbatis版本（从 crud!(SysDict {}); 这种写法看，大概率是rbatis 4.x或更高版本），crud!宏不再生成delete_by_id、select_by_id等按主键ID的专用方法。

旧版本（2.x / 3.x 早期）有 delete_by_id、remove_by_id 等。
新版本（4.x+）简化了 API，只生成少数核心方法：delete_by_map、update_by_map、select_by_map 等。
按 ID 删除现在统一用 delete_by_map + value!{ "id": ... } 来实现（官方推荐方式）。

```shell
cargo test --color=always --package axum-web-app --bin main tests::test_sys_dict_crud
cargo test --color=always --package axum-web-app --bin main tests::test_sys_dict
cargo test --color=always --package axum-web-app --bin main tests --profile test 
```

setup_test_logging不能在cargo test中调用两次

依赖问题：
```shell
ibqodeMBP:axum-web-app ibqo$ cargo update -p cpufeatures --precise 0.2.17
error: There are multiple `cpufeatures` packages in your project, and the specification `cpufeatures` is ambiguous.
Please re-run this command with one of the following specifications:
cpufeatures@0.2.17
cpufeatures@0.3.0
```

尝试解决：
```shell
cargo update cpufeatures@0.3.0 --precise 0.2.17
    Updating crates.io index
error: failed to select a version for the requirement `cpufeatures = "^0.3"`
candidate versions found which didn't match: 0.2.17
location searched: crates.io index
required by package `chacha20 v0.10.0`
    ... which satisfies dependency `chacha20 = "^0.10.0"` (locked to 0.10.0) of package `rand v0.10.0`
    ... which satisfies dependency `rand = "^0.10"` (locked to 0.10.0) of package `rbatis v4.7.2`
    ... which satisfies dependency `rbatis = "^4.5.33"` (locked to 4.7.2) of package `axum-web-app v0.1.0 (/Users/ibqo/Develop/git/github/rust/axum-web-app)`
ibqodeMBP:axum-web-app ibqo$ 
```

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

## Database Profiles

The server selects its database configuration through `APP_PROFILE`. The default is `sqlite`; use `postgresql` or `mysql` to load `config/postgresql.toml` or `config/mysql.toml` respectively. Shared authentication, HTTP, and Redis settings remain in `config/default.toml`.

```bash
APP_PROFILE=sqlite cargo run --bin main
APP_PROFILE=mysql cargo run --bin main
APP_PROFILE=postgresql cargo run --bin main
```

```powershell
$env:APP_PROFILE = 'sqlite'; cargo run --bin main
$env:APP_PROFILE = 'mysql'; cargo run --bin main
$env:APP_PROFILE = 'postgresql'; cargo run --bin main
```

### JWT Authentication and Logs

`POST http://127.0.0.1:9002/api/login` now returns an HS256 JWT in `access_token` (with `session_id` retained as a compatibility alias). The claims contain `sub` (user), `iat` (issued-at), `exp` (expiration), and `jti` (token ID). Protected endpoints require `Authorization: Bearer <access_token>`.

Redis still records the active token, so `POST http://127.0.0.1:9002/api/logout` invalidates the token immediately even before its JWT expiration time. Configure the signing key and lifetime under `[jwt]` in `config/default.toml`; replace the development key before production.

The application logs JWT issuance and validation at `INFO` level. To show the signing/validation flow safely, logs include the algorithm, decoded header, claims, and a short token prefix—but never the signing secret or complete bearer token.

## Usage

Once the application is running, you can access it at `http://127.0.0.1:9002`. You can test the endpoints using tools like `curl` or Postman.

### HTTP Command-Line Smoke Tests

The commands below use the credentials and port in `config/default.toml`. Every request uses a complete HTTP URL.

#### curl (Bash)

```bash
LOGIN_RESPONSE="$(curl --silent --show-error --request POST 'http://127.0.0.1:9002/api/login' \
  --header 'Content-Type: application/json' \
  --data '{"username":"edidada","password":"Pass123Qwe"}')"

ACCESS_TOKEN="$(printf '%s' "$LOGIN_RESPONSE" | sed -n 's/.*"access_token":"\([^"]*\)".*/\1/p')"
test -n "$ACCESS_TOKEN" || { echo "Login failed: $LOGIN_RESPONSE"; exit 1; }

curl --silent --show-error --request GET 'http://127.0.0.1:9002/get' \
  --header "Authorization: Bearer $ACCESS_TOKEN"

curl --silent --show-error --request POST 'http://127.0.0.1:9002/getBody' \
  --header 'Content-Type: application/json' \
  --header "Authorization: Bearer $ACCESS_TOKEN" \
  --data '{"html_code":"<h1>safe</h1><script>alert(1)</script>"}'

curl --silent --show-error --request POST 'http://127.0.0.1:9002/post' \
  --header "Authorization: Bearer $ACCESS_TOKEN" \
  --data-urlencode 'custom_username=tester' \
  --data-urlencode 'custom_password=secret'

curl --silent --show-error --request POST 'http://127.0.0.1:9002/paginate' \
  --header 'Content-Type: application/json' \
  --header "Authorization: Bearer $ACCESS_TOKEN" \
  --data '{"sql":"SELECT id, number, status FROM orders ORDER BY id","page":1,"page_size":2}'

curl --silent --show-error --request POST 'http://127.0.0.1:9002/paginate-with-params' \
  --header 'Content-Type: application/json' \
  --header "Authorization: Bearer $ACCESS_TOKEN" \
  --data '{"sql":"SELECT id FROM orders WHERE status > ?","page":1,"page_size":2,"params":[0]}'

curl --silent --show-error --request POST 'http://127.0.0.1:9002/api/logout' \
  --header "Authorization: Bearer $ACCESS_TOKEN"
```

#### PowerShell (`Invoke-RestMethod`)

```powershell
$loginResponse = Invoke-RestMethod -Method Post -Uri 'http://127.0.0.1:9002/api/login' `
  -ContentType 'application/json' `
  -Body '{"username":"edidada","password":"Pass123Qwe"}'

$accessToken = $loginResponse.access_token
if ([string]::IsNullOrWhiteSpace($accessToken)) {
  throw "Login failed: $loginResponse"
}
$headers = @{ Authorization = "Bearer $accessToken" }

Invoke-RestMethod -Method Get -Uri 'http://127.0.0.1:9002/get' -Headers $headers

Invoke-RestMethod -Method Post -Uri 'http://127.0.0.1:9002/getBody' `
  -Headers $headers -ContentType 'application/json' `
  -Body '{"html_code":"<h1>safe</h1><script>alert(1)</script>"}'

Invoke-RestMethod -Method Post -Uri 'http://127.0.0.1:9002/post' `
  -Headers $headers -ContentType 'application/x-www-form-urlencoded' `
  -Body @{ custom_username = 'tester'; custom_password = 'secret' }

Invoke-RestMethod -Method Post -Uri 'http://127.0.0.1:9002/paginate' `
  -Headers $headers -ContentType 'application/json' `
  -Body '{"sql":"SELECT id, number, status FROM orders ORDER BY id","page":1,"page_size":2}'

Invoke-RestMethod -Method Post -Uri 'http://127.0.0.1:9002/paginate-with-params' `
  -Headers $headers -ContentType 'application/json' `
  -Body '{"sql":"SELECT id FROM orders WHERE status > ?","page":1,"page_size":2,"params":[0]}'

Invoke-RestMethod -Method Post -Uri 'http://127.0.0.1:9002/api/logout' -Headers $headers
```

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

http://127.0.0.1:9002/get
http://127.0.0.1:9002/post
curl -X POST 'http://127.0.0.1:9002/getBody' -H 'Content-Type: application/json' -d '{
"html_code": "a"
}'

cargo run --color=always --package axum-web-app --bin main --profile dev

C:/Users/edida/.cargo/bin/cargo.exe run --color=always --package axum-web-app --bin axum-web-app --profile dev

```powershell
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
```

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

```shell
cargo tree > deps.txt
```
## ammonia v3.3.0库干嘛的？
`ammonia` 是一个用 Rust 编写的 HTML 清理库，版本 3.3.0 是其特定版本。它的主要作用是对输入的 HTML 字符串进行过滤和清理，以确保输出的 HTML 是安全的，避免潜在的安全风险，如跨站脚本攻击（XSS）。以下详细介绍其功能和使用场景：
### 主要功能
#### 1. 过滤危险标签和属性
`ammonia` 可以移除 HTML 中可能包含恶意脚本的标签（如 `<script>`）和危险属性（如 `onclick`、`onload` 等事件处理属性）。例如，以下 HTML 代码包含一个恶意的 `<script>` 标签：
```html
<p>Some text <script>alert('XSS')</script></p>
```
使用 `ammonia` 清理后，`<script>` 标签会被移除，只保留安全的部分：
```html
<p>Some text </p>
```
#### 2. 允许白名单内的标签和属性
你可以配置 `ammonia` 允许特定的 HTML 标签和属性通过过滤。默认情况下，`ammonia` 有一个预定义的白名单，包含常见的安全标签（如 `<p>`、`<a>`、`<img>` 等）和属性。你也可以根据需要自定义白名单。例如，允许 `<a>` 标签的 `target` 属性：
```rust
use ammonia::Builder;

let input = "<a href='https://example.com' target='_blank'>Link</a>";
let cleaned = Builder::default()
   .add_generic_attribute("a", "target")
   .clean(input)
   .to_string();
println!("{}", cleaned);
```

#### 3. 处理 HTML 实体
`ammonia` 会正确处理 HTML 实体，确保它们在清理过程中不会被破坏。例如，`&lt;` 会被保留为 `&lt;`，而不是被错误地解析或修改。
### 使用场景
#### 1. 内容管理系统（CMS）
在 CMS 中，用户可能会输入包含 HTML 代码的内容。为了防止恶意用户注入恶意脚本，需要对用户输入的 HTML 进行清理。`ammonia` 可以确保只有安全的 HTML 内容被保存和显示。
#### 2. 论坛和评论系统
论坛和评论系统通常允许用户使用 HTML 格式化他们的帖子和评论。使用 `ammonia` 可以过滤掉危险的 HTML 代码，保护其他用户免受 XSS 攻击。
#### 3. 网站表单输入
当网站接收用户输入的 HTML 内容（如富文本编辑器的输出）时，使用 `ammonia` 可以对输入进行清理，确保网站的安全性。
### 简单示例代码
以下是一个简单的使用 `ammonia` 清理 HTML 代码的示例：
```rust
use ammonia::clean;

fn main() {
    let input = "<p>Some text <script>alert('XSS')</script></p>";
    let cleaned = clean(input);
    println!("Cleaned HTML: {}", cleaned);
}
```
在这个示例中，`clean` 函数会对输入的 HTML 进行清理，移除危险的 `<script>` 标签，并返回安全的 HTML 字符串。



你给出的是一个使用 `curl` 发送 POST 请求的命令，若要对这个请求进行 AB（Apache Bench）性能测试，需要做如下几步：

### 1. 安装 AB
AB 是 Apache HTTP 服务器自带的一个性能测试工具，若你用的是 Linux 系统，可借助包管理器来安装；要是 macOS 系统，能通过 Homebrew 安装；Windows 系统则可以从 Apache 官网下载 Apache 服务器，AB 工具就在其中。

#### Linux（以 Ubuntu 为例）
```bash
sudo apt-get update
sudo apt-get install apache2-utils
```

#### macOS
```bash
brew install httpd
```

### 2. 准备测试数据文件
因为 AB 不支持直接在命令行里传递 JSON 数据，所以要把 JSON 数据存到一个文件中。创建一个名为 `request.json` 的文件，内容如下：
```json
{
  "html_code": "sssa"
}
```

### 3. 执行 AB 性能测试
使用 AB 命令进行性能测试，下面是具体的命令示例：
```bash
ab -n 1000 -c 100 -p request.json -T application/json -H "Authorization: Bearer 0b6cc057-a6a1-46a6-80e1-5c83ff28af9a" http://127.0.0.1:9002/getBody
```
#### 参数解释：
- `-n 1000`：总共要发送 1000 个请求。
- `-c 100`：并发请求数为 100，也就是同时发送 100 个请求。
- `-p request.json`：指定包含 POST 请求数据的文件。
- `-T application/json`：设置请求的 Content-Type 为 `application/json`。
- `-H "Authorization: Bearer 0b6cc057-a6a1-46a6-80e1-5c83ff28af9a"`：添加请求头 `Authorization`。
- `http://127.0.0.1:9002/getBody`：目标 URL。

### 4. 分析测试结果
执行完 AB 命令后，会输出一系列测试结果，下面是部分关键指标的解释：

```plaintext
Concurrency Level:      100
Time taken for tests:   10.234 seconds
Complete requests:      1000
Failed requests:        0
Total transferred:      234567 bytes
HTML transferred:       200000 bytes
Requests per second:    97.71 [#/sec] (mean)
Time per request:       1023.422 [ms] (mean)
Time per request:       10.234 [ms] (mean, across all concurrent requests)
Transfer rate:          22.33 [Kbytes/sec] received
```
- **Concurrency Level**：并发请求数。
- **Time taken for tests**：整个测试所花费的时间。
- **Complete requests**：成功完成的请求数量。
- **Failed requests**：失败的请求数量。
- **Requests per second**：每秒处理的请求数，数值越高表明性能越好。
- **Time per request**：每个请求平均花费的时间，有两个值，第一个是单个请求的平均时间，第二个是所有并发请求的平均时间。

通过分析这些指标，你就能了解目标服务在特定并发情况下的性能表现。 

Cargo.toml里面

rbatis = "4.5.0"

Cargo.lock
```
[[package]]
name = "rbatis"
version = "4.5.50"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ba14362c725503232be91d999f239c81e5de926b7edf63c2719c1380cfa4e855"
dependencies = [
 "async-trait",
 "dark-std",
 "futures",
 "futures-core",
 "hex",
 "log",
 "parking_lot",
 "rand",
 "rbatis-codegen",
 "rbatis-macro-driver",
 "rbdc",
 "rbdc-pool-fast",
 "rbs",
 "serde 1.0.217",
 "sql-parser",
]
```

cargo update
    Updating crates.io index
error: failed to select a version for the requirement `rbatis = "=4.5.11"`
candidate versions found which didn't match: 4.5.50, 4.5.49, 4.5.48, ...
location searched: crates.io index  https://crates.io/crates/rbatis/4.5.11/dependencies存在，如何更新candidate versions

# 删除本地的 crates.io 索引缓存
Remove-Item -Recurse -Force $env:USERPROFILE\.cargo\git
Remove-Item -Recurse -Force $env:USERPROFILE\.cargo\registry
# 重新运行更新命令
cargo update -p rbatis --precise 4.5.11
```shell
cargo run --package axum-web-app --bin main --profile dev
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
error: rustc 1.80.0 is not supported by the following package:
  generic-array@1.2.0 requires rustc 1.83
Either upgrade rustc or select compatible dependency versions with
`cargo update <name>@<current-ver> --precise <compatible-ver>`
where `<compatible-ver>` is the latest version supporting rustc 1.80.0



thread ThreadId(1), 2025-02-24T05:34:31.163070Z  INFO main: thread ThreadId(1),Listening on 127.0.0.1:9001
thread 'main' panicked at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\hyper-0.14.32\src\server\server.rs:81:13:
error binding to 127.0.0.1:9001: error creating server listener: 以一种访问权限不允许的方式做了一个访问套接字的尝试。 (os error 10013)
stack backtrace:
0: std::panicking::begin_panic_handler
at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library\std\src\panicking.rs:652
1: core::panicking::panic_fmt
at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library\core\src\panicking.rs:72
2: hyper::server::server::impl$1::bind::closure$0
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\hyper-0.14.32\src\server\server.rs:81
3: enum2$<core::result::Result<hyper::server::tcp::AddrIncoming,hyper::error::Error> >::unwrap_or_else<hyper::server::tcp::AddrIncoming,hyper::error::Error,hyper::server::server::impl$1::bind::closure_env$0>
at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9\library\core\src\result.rs:1456
4: hyper::server::server::Server<hyper::server::tcp::AddrIncoming,tuple$<>,enum2$<hyper::common::exec::Exec> >::bind
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\hyper-0.14.32\src\server\server.rs:80
5: main::main::async_block$0
at .\src\main.rs:79
6: tokio::runtime::park::impl$4::block_on::closure$0<enum2$<main::main::async_block_env$0> >
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\park.rs:284
7: tokio::runtime::coop::with_budget
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\coop.rs:107
8: tokio::runtime::coop::budget
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\coop.rs:73
9: tokio::runtime::park::CachedParkThread::block_on<enum2$<main::main::async_block_env$0> >
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\park.rs:284
10: tokio::runtime::context::blocking::BlockingRegionGuard::block_on<enum2$<main::main::async_block_env$0> >
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\context\blocking.rs:66
11: tokio::runtime::scheduler::multi_thread::impl$0::block_on::closure$0<enum2$<main::main::async_block_env$0> >
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\scheduler\multi_thread\mod.rs:87
12: tokio::runtime::context::runtime::enter_runtime<tokio::runtime::scheduler::multi_thread::impl$0::block_on::closure_env$0<enum2$<main::main::async_block_env$0> >,enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::error::Error>,alloc::alloc::G
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\context\runtime.rs:65
13: tokio::runtime::scheduler::multi_thread::MultiThread::block_on<enum2$<main::main::async_block_env$0> >
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\scheduler\multi_thread\mod.rs:86
14: tokio::runtime::runtime::Runtime::block_on_inner<enum2$<main::main::async_block_env$0> >
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\runtime.rs:370
15: tokio::runtime::runtime::Runtime::block_on<enum2$<main::main::async_block_env$0> >
at C:\Users\edida\.cargo\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.43.0\src\runtime\runtime.rs:342
16: main::main
at .\src\main.rs:84
17: core::ops::function::FnOnce::call_once<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::error::Error>,alloc::alloc::Global> > > (*)(),tuple$<> >
at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9\library\core\src\ops\function.rs:250
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\debug\main.exe` (exit code: 101)

Process finished with exit code 101
```
