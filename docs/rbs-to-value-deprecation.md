# `rbs::to_value!` 弃用告警说明

## 告警含义

`rbs::to_value!` 已被 RBS 标记为弃用（deprecated）。它目前仍可使用，
不会改变运行结果，也不是编译错误；该告警提示应迁移到新的宏接口。

## 相关知识点

RBS 是 RBatis 用于承载数据库参数和查询结果的通用值类型。执行带占位符的
SQL 时，Rust 的字符串、整数等参数需要转换为 RBS 的 `Value`，例如：

```rust
rb.exec("INSERT INTO example (number) VALUES (?)", vec![rbs::value!(2)]).await?;
```

这里的 `rbs::value!(2)` 将整数 `2` 转为可绑定到 SQL 参数的 RBS `Value`。
新宏与旧的 `rbs::to_value!` 的用途相同，但 `value!` 是当前受支持的名称。

## 本项目的处理方式

`src/main.rs` 中测试数据插入的全部 `rbs::to_value!` 已替换为
`rbs::value!`，无需额外导入，也不影响 SQL 绑定语义。可使用下面的命令验证：

```powershell
cargo test --bin main --offline
```
