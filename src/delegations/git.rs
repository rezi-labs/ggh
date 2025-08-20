use std::process::exit;

use xshell::cmd;

use crate::base;

pub const GIT: &str = "git";

pub fn commit(message: &str) {
    let msg = format!(r#"{message}"#);
    let sh = base::shell::new();
    if let Err(res) = cmd!(sh, "{GIT} add .").run() {
        eprintln!("{res}");
        exit(1);
    };

    if let Err(res) = cmd!(sh, "{GIT} commit -m {msg}").run() {
        eprintln!("{res}");
        exit(1);
    };
}
