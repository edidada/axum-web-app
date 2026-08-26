# Rust `unsafe` 与所有权移动审计

## 审计结论

对仓库内所有 `.rs` 文件执行关键词审计后，业务源码中没有 `unsafe` block、`unsafe fn`、`unsafe impl` 或 `extern` FFI。`Cargo.toml` 已加入：

```toml
[lints.rust]
unsafe_code = "forbid"
```

因此本 package 的任一 target（主程序、辅助 binary、测试和 benchmark）以后只要直接引入 `unsafe`，编译就会失败。依赖 crate 内部可能使用 `unsafe`，Cargo 的 lint 不会跨 crate 禁止依赖实现；依赖安全应另用版本审计、`cargo audit`/`cargo vet` 和最小依赖策略处理。

`unsafe` 和“移动所有权”不是同一概念：移动是 Rust 日常安全语义，由借用检查器在编译期验证；`unsafe` 是程序员在少数操作上主动承担额外不变量证明责任。当前项目大量使用移动，但无需、也没有使用 `unsafe`。

## 本项目中所有权移动的主要类型

### 1. Axum extractor 把请求数据移入 handler

`src/handlers/login_handler.rs`：

```rust
pub async fn login_handler(
    headers: HeaderMap,
    Extension(rb): Extension<&'static RBatis>,
    Extension(config): Extension<Config>,
    Json(payload): Json<LoginRequest>,
) -> Response
```

`HeaderMap`、`Config` 和 `LoginRequest` 都按值进入函数，handler 获得这些值的所有权。`rb` 本身只是一个共享引用；复制引用不会移动 `RBatis`。这种设计适合异步 handler，因为 future 可以在 `.await` 期间安全持有已拥有的数据。

读取 `payload.username` 做比较不会自动消耗整个 `payload`；但将 `String` 字段直接放入另一个拥有型结构通常会移动该字段。当前代码需要多次使用用户名，因此在签发 claims 时从 `&str` 创建新的 owned `String`。

### 2. 从借用创建 owned 值：`to_owned` / `to_string` / `clone`

`src/jwt.rs`：

```rust
sub: username.to_owned(),
role: role.to_owned(),
jti: Uuid::new_v4().to_string(),
iss: config.issuer.clone(),
aud: config.audience.clone(),
```

claims 必须在 `issue` 返回后独立存在，不能借用函数参数或配置中的 `String`，所以这里需要创建 owned 值：

- `&str::to_owned()` 分配一个新 `String`；
- `Uuid::to_string()` 格式化并分配 `String`；
- `String::clone()` 复制堆上字符串，而不只是复制指针。

这是有意的生命周期边界，不是为了绕开编译器随意 clone。若 claims 改成只在函数内部临时使用，便可考虑借用；但 JWT 序列化和返回值需要 owned 数据。

`src/auth_session.rs` 在构造 SQL 参数时也会 clone claims 字段：

```rust
vec![
    rbs::value!(claims.jti.clone()),
    rbs::value!(claims.sub.clone()),
    rbs::value!(claims.role.clone()),
]
```

这里 `claims` 是共享借用 `&Claims`，不能把其中的 `String` 移走；SQL 参数需要 owned `Value`，因此复制是合理的。更底层的 API 若支持借用参数，才有机会消除这些分配。

### 3. `async move` 把捕获值移入 future

`src/bin/http_sqlite_test.rs` 中测试服务器和并发请求使用：

```rust
let server = tokio::spawn(async move {
    axum::Server::from_tcp(listener)
        .expect("create test server")
        .serve(app.into_make_service())
        .await
        .expect("run test server");
});
```

`tokio::spawn` 的 future 可能比当前栈帧活得更久，通常要求 `Send + 'static`。`move` 把 `listener` 与 `app` 的所有权放进 future，避免 future 借用随后离开作用域的局部变量。移入后，外层不能再使用原值。

并发压测循环先 clone 可共享的句柄和字符串，再分别 move：

```rust
let client = client.clone();
let cookie_header = cookie_header.clone();
tasks.push(tokio::spawn(async move {
    // 每个 task 拥有自己的 Client 句柄和 cookie_header
}));
```

`reqwest::Client::clone()` 只是增加内部共享状态的引用计数，成本远低于复制整个连接池；`String::clone()` 则会复制字符数据。二者虽然都写作 `clone()`，成本模型不同。

`src/routes/mod.rs` 的多个 `async move` handler closure 会把 extractor 已交给 closure 的 `payload` 和复制出来的引用移入其 future。这样数据能跨 `.await`，同时由类型系统阻止悬垂引用。

### 4. 消费型转换：`into_iter` 与 `into_response`

`src/handlers/paginate_handler.rs`：

```rust
let params = params
    .into_iter()
    .map(|value| rbs::value!(value))
    .collect();
```

对 `Vec<String>` 调用 `into_iter()` 会消费 vector，并逐个移动 `String`，因此无需 clone 每个元素。之后不能再使用原 `params`。如果后面仍需要 vector，应使用 `params.iter()` 得到借用，或明确 clone。

项目 handler 中常见的：

```rust
(status, Json(body)).into_response()
```

`IntoResponse::into_response(self)` 接收 `self`，会消费 tuple 和其中的 JSON 值，转成 Axum `Response`。响应体的所有权随后交给 Hyper；这正是“构建后交付”的典型所有权流水线。

### 5. Router、Layer 与应用状态

`src/main.rs`：

```rust
.layer(Extension(config_file.clone()))
.layer(Extension(get_rbatis()))
```

Tower 的 builder 方法通常接收并返回 `self`，所以每次 `.layer(...)` 都消费上一步 builder，再返回新 builder。配置被 clone 后移入 `Extension`，原配置仍用于端口等启动逻辑。数据库使用进程级 `OnceCell<RBatis>`，路由只持有 `&'static RBatis`，没有复制连接池。

这是当前架构中 `Config::clone()` 合理的原因。若以后配置非常大或频繁变更，可改为 `Arc<Config>`，让 clone 只增加引用计数。

### 6. `Option`、模式匹配与部分移动

认证中间件常用：

```rust
let (token, source) = match extract_token(req.headers()) {
    Some(value) => value,
    None => return error(StatusCode::UNAUTHORIZED, "缺少登录凭据"),
};
```

`extract_token` 返回 owned `String`。匹配 `Some(value)` 后，value 从 `Option` 中移动出来，再解构为 `token` 和 `source`。因为临时 `Option` 后续不再使用，不需要 clone。

类似地，`sessions.pop()` 会从 `Vec<AuthSession>` 移出最后一个 session，返回 `Option<AuthSession>`。这比索引后 clone 更直接。

## 常见编译错误如何理解

- `E0382 use of moved value`：值已通过赋值、传参、`into_iter` 或 `move` closure 转移。先判断后续是否真的需要原值，再选择借用、调整语句顺序或 clone。
- `E0505 cannot move out because it is borrowed`：某个借用仍存活时试图移动所有者。缩短借用作用域通常优于 clone。
- `E0515 returns a value referencing local data`：返回值借用了即将销毁的局部值。返回 owned 类型，或让调用方提供生命周期足够长的存储。
- `future is not Send` / closure may outlive current function：spawn 的异步任务捕获了非 `Send` 值或局部借用。可使用线程安全 owned 类型、`Arc`，并在确实需要独立生命周期时使用 `async move`。

## 项目维护约定

1. 默认优先借用；只有跨线程、跨 `.await` 生命周期或需要持久保存时才获取所有权。
2. 每个 `clone()` 应能解释成本与生命周期原因。不要用 clone 盲目压掉借用检查错误。
3. spawn 边界明确使用 `async move`，共享大对象优先 `Arc` 或库本身的廉价句柄 clone。
4. 消费后不再使用的集合优先 `into_iter()`，避免逐元素复制。
5. 保留 package 级 `unsafe_code = "forbid"`。如果未来因 FFI 或底层优化确实需要 unsafe，应放入很小的独立 crate，记录 safety invariant，并配合 Miri、边界测试和 code review，而不是放宽整个 Web 应用的 lint。
