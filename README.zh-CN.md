# mooxide-core

[English](README.md) | 简体中文

用 Rust 重写的 [MOOS](https://oceanai.mit.edu/moos-ivp/pmwiki/pmwiki.php?n=Main.HomePageFull)
（Mission Orientated Operating Suite，机器人发布/订阅中间件），与原版 C++ 实现的线协议双向兼容。

**状态：早期骨架。** workspace 结构、CI、测试与 FFI 脚手架已就位，协议实现尚未开始。

## 项目定位

本项目是我在 AI 指导下学习 Rust 的练习项目，不是可直接用于生产环境的中间件。

这意味着：

- 代码以学习为目标，优先完整、可读地写出实现，而不是先覆盖所有边界情况。
- 接口与 crate 划分随我的学习进度调整，我不承诺语义版本稳定性。
- 已知缺口以 `TODO`、`FIXME` 或 issue 记录，交付时会明确说明哪些部分未完成。
- 兼容性目标仍按线协议逐字节对齐，学习不成为降低验收标准的理由。

## 目标

- **线协议兼容**：`mooxide-db` 接受原版 C++ MOOS 客户端的连接，`mooxide-comms` 客户端能连上原版 `MOOSDB`。
- **现代 Rust**：基于 tokio 的异步网络、`tracing` 日志、`thiserror`/`anyhow` 错误处理。
- **跨平台**：Linux 与 macOS 为一等公民。

## 目录结构

| Crate | 类型 | 用途 |
|---|---|---|
| `mooxide-proto` | lib | 线协议报文类型、编解码、`.moos` 任务文件解析 |
| `mooxide-comms` | lib | 异步客户端库（对应 MOOSLib） |
| `mooxide-db` | bin | 服务端 `mooxide-db`（对应 MOOSDB） |
| `mooxide-apps` | bins | `mooxide-antler`（进程启动器）、`mooxide-logger` |
| `mooxide-ffi` | cdylib/staticlib | C ABI 绑定；头文件 `include/mooxide/mooxide.h` |
| `mooxide-log` | lib | 共享日志初始化，可切换后端（stdout / 文件 / 数据库） |
| `mooxide-test` | lib | 内存 mock 服务端/客户端与报文回放，供测试使用 |
| `xtask` | bin | 任务运行器（`cargo xtask lint \| test \| coverage \| ffi-header`） |

## 开发

```sh
cargo build --workspace
cargo test --workspace
cargo xtask lint        # fmt + clippy + cargo-deny + typos
cargo xtask ffi-header  # 重新生成 include/mooxide/mooxide.h
```

测试分三层：单测（编解码往返）、集成测试（对 spawn 出来的 `mooxide-db` 走真实 TCP）、
互操作性测试（回放原版实现录制的报文，存放在 fixtures；实时互操作手动验证）。

## 许可证

[GPL-3.0-or-later](LICENSE)；库 crate 另可选择 [LGPL-3.0-or-later](COPYING)。© yuhaoxin
