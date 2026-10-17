//! MOOSDB 服务端共享配置类型。`main` 在 `src/main.rs`。

/// 服务端占位配置：连接管理、订阅表落地后由完整配置替换。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    /// 监听地址，默认 `0.0.0.0:9000`。
    pub bind: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bind() {
        let c = ServerConfig {
            bind: "0.0.0.0:9000".into(),
        };
        assert_eq!(c.bind, "0.0.0.0:9000");
    }
}
