use serde::Deserialize;
use xshell::cmd;

use crate::{Format, base};

pub const GH: &str = "gh";
pub const LIMIT: &str = "1000";
pub const FIELDS: &str = "id,url,name,issues";
pub const ISSUE_FIELDS: &str = "url,body,id";

#[derive(Debug, Deserialize)]
struct Repo {
    id: String,
    url: String,
    name: String,
    issues: Issues,
}
#[derive(Debug, Deserialize)]
struct Issues {
    #[serde(alias = "totalCount")]
    total_count: i64,
}

#[derive(Debug, Deserialize)]
struct Issue {
    id: String,
    body: String,
    url: String,
}

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

pub fn find_issue_by_text(org: &str, format: &Format, search: &str) {
    let sh = base::shell::new();

    let repos_res = cmd!(
        sh,
        "{GH} repo list {org} --limit {LIMIT} --json {FIELDS} --no-archived"
    )
    .read();

    let Ok(res) = repos_res else {
        eprintln!("GitHub CLI command failed");
        std::process::exit(1);
    };

    let repos = match serde_json::from_str::<Vec<Repo>>(&res) {
        Ok(repos) => repos,
        Err(err) => {
            println!("{err}");
            println!("{res:?}");
            std::process::exit(1);
        }
    };

    let res = repos
        .iter()
        .filter(|r| r.issues.total_count > 0)
        .map(|repo| {
            let repo_name = repo.name.clone();
            let issue = cmd!(
                sh,
                "{GH} issue list -R {org}/{repo_name} --limit {LIMIT} --json {ISSUE_FIELDS} --state all"
            ).read();

            let Ok(res) = issue else {
                eprintln!("GitHub CLI command failed");
                std::process::exit(1);
            };

            let issue = match serde_json::from_str::<Vec<Issue>>(&res) {
                Ok(repos) => repos,
                Err(err) => {
                    println!("{err}");
                    println!("{res:?}");
                    std::process::exit(1);
                }
            };
            issue
        }).flatten()
        .collect::<Vec<Issue>>();

    println!("{res:?}")
}
