//! 测试辅助 crate：供各 crate 的测试与集成测试作 dev-dependency 使用。
//!
//! 提供内存 mock 服务端/客户端（不产生 socket IO）与报文回放器；
//! 协议规格落地后在此实现与 `mooxide-proto` 编解码配套的测试设备。

/// 内存 mock 占位：记录被测代码注入的报文文本。
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MockMailbox {
    /// 已收到的报文原文。
    pub received: Vec<String>,
}

impl MockMailbox {
    /// 记录一条报文。
    pub fn push(&mut self, raw: impl Into<String>) {
        self.received.push(raw.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mailbox_records() {
        let mut m = MockMailbox::default();
        m.push("MOOS");
        assert_eq!(m.received, ["MOOS"]);
    }
}
