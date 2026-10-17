//! 统一日志初始化。所有二进制在 `main` 里调用 [`init`]，
//! 保证各进程的日志格式与 `RUST_LOG` 过滤行为一致。
//!
//! 后端通过 [`LogConfig`] 选择；当前实现 stdout，
//! File 与 Database 变体见结构体文档中的缺口说明。

use std::path::PathBuf;

/// 日志后端配置。
#[derive(Debug, Clone)]
pub enum LogConfig {
    /// 打印到标准输出。
    Stdout,
    /// 写入指定路径的文件（按日滚动）。
    File(PathBuf),
    /// 写入数据库。
    Database {
        /// 数据库连接描述。
        dsn: String,
    },
}

/// 初始化全局日志。重复调用会覆盖已有全局 subscriber。
///
/// # 缺口
/// - `File` 变体：tracing-appender 的滚动 writer 需要持有 guard
///   防止日志丢失，接口需要返回 guard 或改用后台线程刷新；
///   触发条件：首个 bin 真正需要文件日志时实现。
/// - `Database` 变体：需要先有存储层（SQLite 或复用 MOOSDB 自身）；
///   触发条件：存储层规格落地后实现。
pub fn init(config: &LogConfig) -> anyhow::Result<()> {
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    match config {
        LogConfig::Stdout => {
            tracing_subscriber::fmt().with_env_filter(filter).init();
        }
        LogConfig::File(_) | LogConfig::Database { .. } => {
            // 骨架阶段仅实现 Stdout，其余变体按 init 文档中的缺口处理。
            tracing_subscriber::fmt().with_env_filter(filter).init();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_stdout_is_ok() {
        assert!(init(&LogConfig::Stdout).is_ok());
    }
}
