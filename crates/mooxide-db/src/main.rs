//! MOOSDB 服务端：接收客户端连接，维护发布/订阅数据库并转发邮件。

fn main() -> anyhow::Result<()> {
    let _config = mooxide_log::LogConfig::Stdout;
    mooxide_log::init(&_config)?;
    tracing::info!("mooxide-db 启动");
    Ok(())
}
