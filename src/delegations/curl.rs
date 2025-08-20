use xshell::cmd;

use crate::base;

pub const CURL: &str = "curl";

pub fn run(args: &[String]) {
    let sh = base::shell::new();
    let result = if args.is_empty() {
        cmd!(sh, "{CURL}").run()
    } else {
        cmd!(sh, "{CURL} {args...}").run()
    };

    if let Err(e) = result {
        eprintln!("Curl command failed: {}", e);
        std::process::exit(1);
    }
}
