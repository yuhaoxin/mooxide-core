//! MOOS 线协议报文类型与编解码。
//!
//! 本 crate 只依赖标准库，负责报文与结构体之间的双向转换；
//! 网络收发在 `mooxide-comms`，进程内 mock 在 `mooxide-test`。
//!
//! 协议事实以 `docs/protocol/` 下的笔记为准；可参考原版源码理解协议，
//! 不复制其文本进本仓库。

/// 占位类型：协议实现落地后由真实报文枚举替换。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placeholder {
    /// 报文原文本，实现落地前仅原样保留。
    pub raw: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_roundtrip() {
        let p = Placeholder {
            raw: "MOOS".to_string(),
        };
        assert_eq!(p.raw, "MOOS");
    }
}
