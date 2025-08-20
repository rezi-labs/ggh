use xshell::cmd;

use crate::base;

pub const GIT: &str = "git";

pub fn run(args: &[String]) {
    let sh = base::shell::new();
    let result = if args.is_empty() {
        cmd!(sh, "{GIT}").run()
    } else {
        cmd!(sh, "{GIT} {args...}").run()
    };

    if let Err(e) = result {
        eprintln!("Git command failed: {}", e);
        std::process::exit(1);
    }
}
