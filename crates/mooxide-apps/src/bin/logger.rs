//! pLogger 对应物：订阅指定变量并把数据落盘。

fn main() -> anyhow::Result<()> {
    let _config = mooxide_log::LogConfig::Stdout;
    mooxide_log::init(&_config)?;
    tracing::info!("mooxide-logger 启动");
    Ok(())
}
