//! xtask 子命令实现。

use std::process::Command;

/// 在仓库根目录执行外部命令，失败时返回带命令名的错误。
fn exec(program: &str, args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|e| anyhow::anyhow!("无法启动 {program}: {e}"))?;
    if !status.success() {
        anyhow::bail!("{program} {} 退出码 {status}", args.join(" "));
    }
    Ok(())
}

/// 按调度入口分发子命令。
pub fn run(task: Option<&str>, _rest: &[String]) -> anyhow::Result<()> {
    match task {
        Some("lint") => lint(),
        Some("test") => test(),
        Some("coverage") => coverage(),
        Some("ffi-header") => ffi_header(),
        other => anyhow::bail!("未知命令 {other:?}，可用：lint | test | coverage | ffi-header"),
    }
}

/// 格式化 + clippy + cargo-deny + typos 全量检查。
fn lint() -> anyhow::Result<()> {
    exec("cargo", &["fmt", "--all", "--", "--check"])?;
    exec(
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    exec("cargo", &["deny", "--workspace", "check"])?;
    // typos 未安装时跳过：本地开发不强制装，CI 单独装。
    if std::env::var_os("XTASK_REQUIRE_TYPOS").is_some() || Command::new("typos").output().is_ok() {
        exec("typos", &[])?;
    }
    Ok(())
}

/// 全量测试。
fn test() -> anyhow::Result<()> {
    exec("cargo", &["test", "--workspace"])?;
    Ok(())
}

/// 覆盖率报告：需要 cargo-llvm-cov，只生成不设阈值。
fn coverage() -> anyhow::Result<()> {
    exec(
        "cargo",
        &[
            "llvm-cov",
            "--workspace",
            "--lcov",
            "--output-path",
            "lcov.info",
        ],
    )?;
    println!("覆盖率报告已写入 lcov.info");
    Ok(())
}

/// 用 cbindgen 重新生成 C 头文件到 include/mooxide/mooxide.h。
fn ffi_header() -> anyhow::Result<()> {
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        ..Default::default()
    };
    let bindings = cbindgen::Builder::new()
        .with_config(config)
        .with_crate("crates/mooxide-ffi")
        .generate()
        .map_err(|e| anyhow::anyhow!("cbindgen 失败: {e}"))?;
    let path = std::path::Path::new("include/mooxide/mooxide.h");
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    // write_to_file 返回 false 表示内容未变化或 noop，都是成功。
    bindings.write_to_file(path);
    println!("头文件已生成: {}", path.display());
    Ok(())
}
