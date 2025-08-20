use clap::{Parser, Subcommand, ValueEnum, command};
use xshell::cmd;

use crate::delegations::git;

mod base;
mod delegations;

#[derive(Parser)]
#[command(name = "ggh")]
#[command(about = "A CLI tool with git, gh, and curl commands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Human,
    Json,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Commit")]
    Commit {
        #[arg(short, group = "input")]
        message: String,
    },
    #[command(about = "get all repos of an org")]
    Repos {
        #[arg(value_enum, short, long, default_value = "human")]
        format: Format,
        #[arg(short, long)]
        org: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Commit { message } => {
            git::commit(message);
        }
        Commands::Repos { format, org } => delegations::gh::get_all_repos(org, format),
    }
}
