//! MOOS 应用集合的共享配置类型：antler 进程启动器、logger 数据记录器等。
//! 各 bin 的 `main` 在 `src/bin/` 下。

/// antler 占位配置：任务文件解析落地后由完整配置替换。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AntlerConfig {
    /// 任务文件（.moos）路径。
    pub mission_file: String,
}

/// logger 占位配置：订阅管理与落盘策略落地后由完整配置替换。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoggerConfig {
    /// 输出文件路径。
    pub output: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configs_keep_values() {
        let a = AntlerConfig {
            mission_file: "foo.moos".into(),
        };
        assert_eq!(a.mission_file, "foo.moos");
        let l = LoggerConfig {
            output: "log.txt".into(),
        };
        assert_eq!(l.output, "log.txt");
    }
}
