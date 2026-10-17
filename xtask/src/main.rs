//! CI 与本地共用的检查入口：`cargo xtask <命令>`。
//! 命令表见 `main`；实现细节在 `src/tasks/`。

mod tasks;

use std::process::ExitCode;

fn main() -> ExitCode {
    let task = std::env::args().nth(1);
    let rest = std::env::args().skip(2).collect::<Vec<_>>();
    match tasks::run(task.as_deref(), &rest) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("xtask: {msg}");
            ExitCode::FAILURE
        }
    }
}
