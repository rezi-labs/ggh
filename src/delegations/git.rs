use xshell::cmd;

use crate::base;

pub const GIT: &str = "git";

pub fn commit(message: &str){
    let msg = format!(r#"{message}"#);
    let sh = base::shell::new();
    cmd!(sh, "{GIT} add .").run().unwrap();
    cmd!(sh, "{GIT} commit -m {msg}").run().unwrap();
}