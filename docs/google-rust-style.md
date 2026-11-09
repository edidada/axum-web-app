# 对照 Google Rust Style Guide 的代码改动说明

本分支 `google_rust_style` 对照
[Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html)
（本地存档：`D:\develops\git\github\langnote\Rust Style Guide _ Google Style Guides.mhtml`）
对代码做了整理，并新增了本分支专用的 CI 工作流。本文逐条说明**改了什么、对应规范哪一条、以及为什么这样改**。

> 说明：这份指南面向 Google 内部（Chrome / Fuchsia / Android 等）代码库，其中标注
> `[chrome]` / `[Fuchsia]` 的条目只适用于对应代码库，本项目不采用。本次挑选的是
> 通用、可落地、且不改变运行时行为或公开 API 的条目。

---

## 1. 代码组织：禁止 glob 导入、顶层函数用模块名限定

**规范条目**

- `Don't use glob imports`
- `Qualify calls to top-level functions with a module name`
- `Type names should be imported and used unqualified`

**改动**

| 文件 | 改动前 | 改动后 |
| --- | --- | --- |
| `src/routes/mod.rs` | `use crate::handlers::crud_handler::*;` 与 `use crate::handlers::report_handler::*;` | `use crate::handlers::crud_handler;` / `use crate::handlers::report_handler;`，48 个 CRUD 处理函数与 6 个报表函数改为 `crud_handler::xxx(...)`、`report_handler::xxx(...)` 调用 |
| `src/domain/table/join_query.rs` | `use crate::domain::dto::join_dto::*;` | 显式列出 7 个 DTO 类型 |

**为什么这样改**

- glob 导入让读者无法凭名字判断一个符号来自哪个模块，也会给大规模自动重构（LSC）
  带来额外成本；指南明确要求禁止（`use super::*` 用于内联测试模块属于指南允许的例外，
  本次保留）。
- 指南对「函数」和「类型」区别对待：**类型**导入后不加限定（`SysDict`），
  **顶层函数**则导入模块后用 `模块名::函数名()` 调用。所以 `routes/mod.rs`
  没有把 48 个函数逐个写进一个巨大的 `use {...}` 列表，而是导入模块并限定调用，
  既消除 glob 又保持可读。
- 注意 `routes/mod.rs` 里原来由 glob 带入的 `SysDictPageRequest` 类型也一并补成了
  显式导入，否则编译不过——这正是 glob 导入「藏依赖」缺点的实际体现。

---

## 2. 代码组织：一条 `use` 只导入同一个模块的内容

**规范条目**

- `A use statement should import items from one module`
  （对应 rustfmt 的 `imports_granularity = "Module"`：花括号内不应再出现 `::`）

**改动**

把形如 `use axum::{response::Json, extract::Extension, http::StatusCode};` 的
混合导入拆成按模块分组的多条 `use`。涉及：

`src/main.rs`、`src/routes/mod.rs`、`src/domain/table/paginate.rs`、
`src/domain/table/join_query.rs`、`src/handlers/{crud,get,logout,paginate}_handler.rs`、
`src/middleware/request_logging.rs`、`src/bin/{http_sqlite_test,log_app,log_chinese}.rs`
等文件。

**为什么这样改**

一条 `use` 只对应一个模块时，增删依赖时改动最小、合并冲突最少，也让 IDE 与
代码检索更容易定位符号来源。该条目对应的 rustfmt 选项是 nightly-only，
本项目在 stable 上构建，因此**手工落实**这一规则，而不是依赖 rustfmt 自动拆分
（见第 6 节）。

---

## 3. 实现细节：用 `expect` 取代 `unwrap`

**规范条目**

- `Prefer expect over unwrap`：如果必须 panic，`expect` 应写清原因。

**改动**

- `src/main.rs`：CORS 静态 origin 解析、测试里的内存 SQLite 初始化；
- `src/bin/rbatis/rbatis_main.rs`、`rbatis_xml_main.rs`、`rbatis_xml_main2.rs`：
  建日志目录、创建日志文件、初始化 logger；
- 新增/重写后的测试统一使用 `expect("原因")`。

改完后 `src/` 与 `tests/` 中不再有裸 `.unwrap()`；`unwrap_or` /
`unwrap_or_default` / `unwrap_or_else` 属于有回退的非 panic 操作，按规范保留。

**为什么这样改**

`unwrap()` 与 `expect()` 行为完全相同，但 `expect` 在 panic 信息里留下「为什么这里
不可能失败」的说明。线上排障时，`called Option::unwrap() on a None` 几乎无用，
而 `the static CORS origin is a valid header value` 能立刻指明是常量写错。

---

## 4. 实现细节：断言补充上下文字符串

**规范条目**

- `Use context strings to clarify assertions`

**改动**

`src/handlers/health_handler.rs`、`post_handler.rs`、`paginate_handler.rs`、
`src/config.rs`、`src/main.rs` 以及新测试里的裸 `assert!` / `assert_eq!`
都补上了说明性消息，失败时带上实际值（如 `got {body:?}`）。

**为什么这样改**

断言失败信息会出现在日志里。写明「期望什么、为什么、实际值是什么」，
能区分到底是调用方用错 API 还是实现有 bug，比重复条件的注释更有用。

---

## 5. 文档与 API 标注

**规范条目**

- `When and how to write documentation`（`///` 标注条目，`//!` 标注模块）
- `#[must_use] on functions`（纯函数被忽略返回值通常意味着 bug）

**改动**

- 为 `src/main.rs`、`src/config.rs`、`src/jwt.rs`、
  `src/handlers/health_handler.rs`、`src/domain/table/join_query.rs` 增加
  `//!` 模块说明与 `///` 公开条目说明；`get_rbatis()` 补了 `# Panics` 段，
  因为它依赖 `init_rbatis()` 先执行。
- `src/jwt.rs` 的纯函数 `token_preview()` 标注 `#[must_use]`：调用它却丢掉返回值
  一定是写错了。

**为什么这样改**

`///` / `//!` 是 AST 的一部分，能被 rustdoc、IDE 与静态检查消费；普通 `//` 只是
给人看的注释。指南同时强调「注释要说代码没说出来的事」，所以只补充「为什么」
与「前置条件」，不复述函数名。

---

## 6. 格式化：使用 rustfmt 并固化配置

**规范条目**

- `Use the appropriate formatting tool`
- 配置建议：`max_width = 100`、`hex_literal_case = Lower`、
  `format_macro_matchers = true`、`imports_granularity = Module`

**改动**

- 新增 `rustfmt.toml`，写入 `edition = "2021"` 与 `max_width = 100`。
- 对本次改动到的文件执行了 rustfmt。

**为什么这样改 / 为什么没全启用**

指南明确允许「为了能在 stable 上构建而跳过 nightly-only 的 rustfmt 选项」。
`imports_granularity`、`hex_literal_case`、`format_macro_matchers` 都是
nightly-only，在 stable 上只会产生 `unstable option` 警告并被忽略，因此
**只在 `rustfmt.toml` 里以注释记录、不强行启用**；其中
`imports_granularity = Module` 已按第 2 节手工落实。

CI 里也**没有**加 `cargo fmt --all -- --check`：不同 stable 版本的 rustfmt
输出可能略有差异，把它作为硬门禁会让构建因为工具链升级而非代码问题随机变红。

---

## 7. 测试：把不可靠的 HTTP 集成测试改成确定性测试

**背景（改动前 `cargo test` 是失败的）**

`tests/http_tests.rs` 原来在 detached 线程里手工解析 `config/default.toml`、
拼 MySQL DSN 并启动一个 mock server，再用硬编码的 500 ms 睡眠等待端口就绪。
而现在的 `config/default.toml` 已经没有 `[mysql]` 段（配置重构为
`default.toml` + `APP_PROFILE` 指定的 profile 文件），
于是 `toml_value["mysql"]["username"]` 直接 panic，线程死掉、端口没监听，
5 个测试全部 `ConnectionRefused`。基线 `cargo test` 退出码 101。

**改动**

重写 `tests/http_tests.rs`：用 `tower::ServiceExt::oneshot` 把请求在**进程内**
直接喂给 `axum::Router`，不绑定端口、不读配置文件、不依赖数据库 / Redis /
时序。6 个测试覆盖 `/login`、`/get`、`/post`、`/getBody`、
`/api/sys-dict/list`、`/health` 的状态码与响应体。

**为什么这样改**

- 测试的可靠性优先：不引入 socket、端口冲突、启动竞态和外部服务。
- 真实运行中的 HTTP 接口验证放在 CI 的独立步骤（见第 8 节），两者分工明确：
  Rust 测试保证路由逻辑，CI 保证编译产物确实能监听并响应 HTTP。
- 新测试本身也按指南写：`expect` 带原因、断言带上下文。

---

## 8. CI：新增 `google_rust_style` 分支工作流

新增 `.github/workflows/google-style-ci.yml`，触发条件为
`push` / `pull_request` 到 `google_rust_style` 以及 `workflow_dispatch`（便于手动重跑）。

**作业步骤与理由**

| 步骤 | 理由 |
| --- | --- |
| `actions/checkout@v4` + `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2` | 现代且维护中的 action；缓存加速后续构建 |
| `apt-get install libsqlite3-dev libssl-dev pkg-config` | `Cargo.lock` 里 `libsqlite3-sys` 通过 `pkg-config` 链接系统 SQLite；Linux runner 需保证头文件/库存在 |
| `cargo build --verbose` | 目标 1：编译成功 |
| `cargo test --verbose` | 目标 2：单元测试 + 集成测试全部通过 |
| 启动 `./target/debug/main` 并轮询 `/health` | 目标 3：HTTP 接口成功 |

**HTTP 检查的设计**

- 用 30 次 × 1 秒的**就绪轮询**取代固定 `sleep`，避免启动慢时误判失败；
- 依次请求 `/health`、`/health/detailed`、`/ready`、`/live`，
  用 `curl -fsS` 断言 200，并 `grep` 校验响应体里的
  `"status":"healthy"`、`"database_connected":true`、`"ready":true`、`"alive":true`；
- 选这四个端点是因为它们**不经过 `validate_session`、不查数据库、不需要 Redis**，
  是依赖最少、结果最确定的接口；带鉴权的 `/api/login` 需要 Redis 会话，
  由本地 `test_api.ps1` / e2e 测试覆盖，不放进这个最小 CI；
- 用 `trap` 在步骤结束时 kill 服务进程并打印 `server.log`，失败时便于排查。

**关于 `Cargo.lock`**

`Cargo.lock` 原本在 `.gitignore` 里。本项目是**二进制应用**（不是库），
按 Rust 惯例应提交 lockfile；更重要的是不提交时 CI 会重新解析依赖，
可能拿到与本地验证不同的版本。本次把 `Cargo.lock` 从 `.gitignore` 移除并纳入版本控制，
使 CI 与本地使用**同一套已验证可编译的依赖版本**。

---

## 9. 本地验证方式

```bash
cargo build
cargo test

# 启动真实服务（默认 profile = sqlite，端口 9002）
cargo run --bin main

# 另开一个终端
curl -fsS http://127.0.0.1:9002/health
curl -fsS http://127.0.0.1:9002/health/detailed
curl -fsS http://127.0.0.1:9002/ready
curl -fsS http://127.0.0.1:9002/live
```

---

## 10. 本次未处理 / 后续可选

- `src/xml/mod.rs` 目前没有被任何 `mod` 声明引用，且其依赖的 `xml` crate
  也不在 `Cargo.toml` 中，属于残留的死代码；本次不动它，避免把「死代码」误当成
  已验证的改动。若要启用，可另行接回并再按 `new()` → `Default`、
  `Option<&String>` → `Option<&str>` 等条目整理。
- `src/infrastructure/minio_client.rs::new()` 返回 `Result`，按
  `Naming initialization functions` 应叫 `try_new()`；它在
  `#[cfg(feature = "minio")]` 下、默认不编译，本次未改。
- 全仓库范围的 `cargo fmt --all` 未执行（只格式化了本次涉及的文件），
  以保持本分支 diff 聚焦；如需全量统一可作为独立提交。
- 指南中 `Avoid Rc and Arc`、`Don't use async/.await [chrome]` 等条目与
  axum/tokio 应用的架构强绑定或仅适用于 Chrome，本次不强行套用。
