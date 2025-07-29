# 模块



模块命名限制：
- 以字母开头，可以包含数字和下划线。
- 不可使用特殊字符。


## 命名约定
自定义模块不能与标准库中的模块冲突。例如： `core`, `std`。

若模块命名为`core`,则会出现以下错误。
```rust
error[E0433]: failed to resolve: could not find marker in core
 --> crates/domain/src/repository/monitor_repository.rs:8:1
  |
8 | #[async_trait]
  | ^^^^^^^^^^^^^^ could not find marker in core

```