use xshell::cmd;

use crate::{Format, base};

pub const GH: &str = "gh";
pub const LIMIT: &str = "1000";
pub const FIELDS: &str = "url,name";

pub fn get_all_repos(org: &String, format: &Format) {
    let sh = base::shell::new();

    let res = match format {
        Format::Human => cmd!(sh, "{GH} repo list {org} --limit {LIMIT}").read(),
        Format::Json => cmd!(sh, "{GH} repo list {org} --limit {LIMIT} --json {FIELDS}").read(),
    };

    match res {
        Err(e) => {
            eprintln!("GitHub CLI command failed: {e}");
            std::process::exit(1);
        }
        Ok(s) => {
            println!("{s}")
        }
    }
}
