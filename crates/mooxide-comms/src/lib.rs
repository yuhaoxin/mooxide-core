//! MOOS 异步客户端通信库，对应原版 MOOSLib。
//!
//! 基于 tokio 提供到 MOOSDB 的 TCP 连接、注册与发布/订阅；
//! 报文编解码复用 `mooxide-proto`。

/// 占位类型：客户端接口在协议规格落地后定义。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placeholder {
    /// 目标 MOOSDB 地址，如 `localhost:9000`。
    pub address: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_address() {
        let c = Placeholder {
            address: "localhost:9000".into(),
        };
        assert_eq!(c.address, "localhost:9000");
    }
}
