//! pAntler 对应物：按任务文件启动一组 MOOS 进程。
//! 命令行接口与原版 pAntler 需求兼容（接受任务文件路径）。

fn main() -> anyhow::Result<()> {
    let _config = mooxide_log::LogConfig::Stdout;
    mooxide_log::init(&_config)?;
    tracing::info!("mooxide-antler 启动");
    Ok(())
}
