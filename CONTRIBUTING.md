# 贡献指南

## 协议与版权

本项目采用双授权：整体 GPL-3.0-or-later（`LICENSE`），库 crate 额外按
LGPL-3.0-or-later 提供（`COPYING` 附全文）。提交贡献即同意作品按此双授权发布。

## 编码规则

- **lint 基线**：`cargo xtask lint` 必须零告警（rustfmt 默认配置、
  clippy `-D warnings`、`unwrap_used`/`expect_used`/`undocumented_unsafe_blocks`
  均为 deny，cargo-deny 检查依赖许可证，typos 检查拼写）。
- **unsafe**：全仓库 deny，唯一例外 `mooxide-ffi`——C ABI 边界必须有
  `// SAFETY:` 注释说明前提。
- **公开 API**：库 crate 的公开项必须有文档注释（rustdoc）。
- **错误处理**：库 crate 用 `thiserror` 定义错误枚举；bin crate 用 `anyhow`。
  禁止静默吞错；信任边界（网络输入、配置文件）处必须校验并返回明确错误。
- **日志**：只 emit `tracing` 事件，初始化统一走 `mooxide_log::init`，
  不要在库 crate 里安装 subscriber。
- **测试**：新逻辑必须带失败即可复现的测试；协议实现优先使用
  `mooxide-test` 的 mock 与 fixtures 回放，不依赖外部环境。
- **占位删除**：`Placeholder` 类型的模块被真实实现替换时，占位与其测试一并删除。
- **原版代码**：参考原版 MOOS 源码理解协议时，不复制其文本进本仓库；
  协议事实（报文格式、语义）以 `docs/protocol/` 下的笔记与 fixtures 为准。

## 提交规范

Conventional Commits，中文 subject ≤ 50 字符：
`feat(proto): 报文解码器` / `fix(comms): 重连时的注册丢失`。
`type` ∈ feat|fix|refactor|perf|test|chore|revert|style|docs。
提交信息不含 AI 署名或工具品牌信息。

## 分支与同步

- 主开发分支 `main`，GitHub（github.com/yuhaoxin/mooxide-core）为主仓库。
- GitLab（`mooxide` 组下 `mooxide-core`，私有）为只读镜像，由 GitHub Actions
  的 `mirror.yml` 自动推送同步，不要直接向 GitLab 提交。
