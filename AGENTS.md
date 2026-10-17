# Repository Guidelines

MOOS（Mission Orientated Operating Suite，机器人发布/订阅中间件）的 Rust 重实现，与原版 C++ MOOS **线协议双向兼容**：`mooxide-db` 接受原版客户端连接，`mooxide-comms` 客户端可连原版 MOOSDB。tokio 异步模型，Linux/macOS 一等公民。当前为骨架阶段：workspace 结构、CI、FFI、日志就位，协议实现尚未开始。双授权：整体 GPL-3.0-or-later，库 crate 额外 LGPL-3.0-or-later。

## Project Overview

## Architecture & Data Flow

依赖分层（数据自底向上）：

```
mooxide-proto   线协议编解码，零依赖、无 IO
   ↑
mooxide-comms   异步客户端（tokio TCP、注册、发布/订阅）
   ↑
mooxide-db (bin)     服务端枢纽：接受连接、维护订阅表、转发邮件
mooxide-apps (2 bins) mooxide-antler 进程启动器 / mooxide-logger 记录器

mooxide-ffi    cdylib+staticlib，C ABI 边界，只依赖 proto
mooxide-test   mock 设施（dev 用，目前仅 MockMailbox）
mooxide-log    日志初始化，被 db/apps 的 main 调用；库 crate 只 emit 不装 subscriber
xtask          任务运行器，CI 与本地共用入口
```

协议事实（报文格式、语义）记录在 `docs/protocol/` 的笔记与 fixtures 中；可参考原版源码理解协议，但不复制其文本进本仓库。

## Key Directories

| 路径 | 用途 |
|---|---|
| `crates/mooxide-proto` | 报文类型与编解码；将来含 `.moos` 任务文件解析 |
| `crates/mooxide-comms` | 客户端库 |
| `crates/mooxide-db` | 服务端，`src/lib.rs` 共享配置 + `src/main.rs` 入口 |
| `crates/mooxide-apps` | 多 bin 应用；共享类型在 `src/lib.rs`，入口在 `src/bin/{antler,logger}.rs` |
| `crates/mooxide-ffi` | C ABI；头文件产物 `include/mooxide/mooxide.h` |
| `crates/mooxide-log` | `LogConfig`（Stdout/File/Database）+ `init` |
| `crates/mooxide-test` | mock server/client、报文回放（已声明未实现） |
| `docs/protocol/` | 协议笔记与报文样本（待建立） |
| `xtask/src/tasks.rs` | 四个任务命令的全部实现 |

## Development Commands

必须从仓库根运行（xtask 的 `exec` 继承 CWD）：

```sh
cargo build --workspace      # 构建
cargo xtask lint             # fmt --check + clippy -D warnings + cargo-deny + typos（未装则跳过）
cargo xtask test             # = cargo test --workspace
cargo xtask coverage         # cargo-llvm-cov 生成 lcov.info，无阈值
cargo xtask ffi-header       # cbindgen 重新生成 include/mooxide/mooxide.h
```

CI（`.github/workflows/ci.yml`，ubuntu+macos）：test 矩阵、lint、coverage、`ffi-header-fresh`（重新生成后 `git diff --exit-code` 强制头文件同步）。GitLab（`.gitlab-ci.yml`）为镜像侧精简 CI，只跑 test + clippy。

## Code Conventions & Common Patterns

- **错误处理**：库 crate 用 `thiserror` 枚举，bin crate 用 `anyhow`；`unwrap()`/`expect()` 被 workspace lint deny，用 `?` 传播；信任边界（网络输入、配置文件）必须显式校验。
- **unsafe**：全仓库 `unsafe_code = "deny"`，唯一例外 `mooxide-ffi`（C ABI 必须 unsafe，每个 unsafe 块带 `// SAFETY:` 注释）。注意 `mooxide-ffi` 用 `[lints.rust]`/`[lints.clippy]` 平铺而非 `workspace = true`，三条 clippy deny 需手动重列。
- **日志**：库 crate 只 `tracing::info!` 等 emit；subscriber 安装统一走 `mooxide_log::init(&LogConfig)`，`main` 里调一次。`init` 重复调用会 panic（全局 subscriber 冲突）。
- **异步**：tokio（features: `macros`, `rt-multi-thread`）。骨架阶段尚无 async 代码，测试将来用 `#[tokio::test]`。
- **新 crate/依赖**：依赖版本统一加根 `Cargo.toml` 的 `[workspace.dependencies]`，crate 内用 `xxx.workspace = true`；crate 间依赖写显式 `{ path = "..", version = "0.1.0" }`。新依赖的许可证必须属于 `deny.toml` 白名单。
- **占位约定**：`Placeholder`/`*Config` 类型是骨架占位，被真实实现替换时占位与其测试一并删除。当前占位：`mooxide-proto::Placeholder`、`mooxide-comms::Placeholder`、`mooxide_db::ServerConfig`、`mooxide_apps::{AntlerConfig, LoggerConfig}`。
- **提交**：Conventional Commits，中文 subject ≤ 50 字符，如 `feat(proto): 报文解码器`；不提交到 GitLab（只读镜像）。

## Important Files

- `Cargo.toml` — workspace 成员、共享依赖、`[workspace.lints]` 基线
- `crates/mooxide-ffi/src/lib.rs` — FFI 范例：`#[unsafe(no_mangle)]` + `// SAFETY:` 模式（edition 2024 语法）
- `include/mooxide/mooxide.h` — cbindgen 生成物，改动 FFI 导出后必须 `cargo xtask ffi-header` 并提交
- `xtask/src/tasks.rs` — 命令入口，新增任务在此 dispatch
- `deny.toml` — 许可证白名单
- `LICENSE`（GPL-3.0）/ `COPYING`（LGPL-3.0）— 双授权文本

## Runtime/Tooling Preferences

- Rust stable（`rust-toolchain.toml` 滚动跟踪，不钉版本），edition 2024，resolver 3。
- 外部工具：`cargo-deny`、`typos`、`cargo-llvm-cov` 本地未装不阻塞（xtask 的 typos 缺失时静默跳过；CI 单独装）。强制本地 typos 设 `XTASK_REQUIRE_TYPOS=1`。
- 无 rustfmt.toml/clippy.toml——格式化用 rustfmt 完全默认。

## Testing & QA

- **现状**：仅内嵌单测（各 lib 的 `src/lib.rs` 尾部 `#[cfg(test)] mod tests`），每 crate 一个冒烟测试；无 `tests/` 集成目录、无 fixtures、无 doc test。
- **测试风格**：函数名不用 `test_` 前缀，直接描述断言的事实（如 `default_bind`、`mailbox_records`）；`assert_eq!(实际, 期望字面量)`；测试代码同样受 `unwrap_used` deny 约束，用 `.into()`/`unwrap_or_default()`。
- **目标三层**（README 已声明，待落地）：单测（编解码 round-trip）、集成测试（spawn `mooxide-db` 走真 TCP）、互操作（`fixtures/` 报文回放）。协议实现优先用 `mooxide-test` 的 mock，不依赖外部环境。
- **要求**：新逻辑必须带"预期回归出现时会失败"的测试。
- **覆盖率**：`cargo xtask coverage` 出报告（lcov.info，gitignored），CI 上传 artifact，不设阈值。
