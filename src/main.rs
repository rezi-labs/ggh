use clap::{Parser, Subcommand, ValueEnum, command};

use crate::delegations::git;

mod base;
mod delegations;

#[derive(Parser)]
#[command(name = "ggh")]
#[command(about = "A CLI tool that simplifies git and github")]
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
    Issue {
        #[arg(value_enum, short, long, default_value = "human")]
        format: Format,
        #[arg(short, long)]
        org: String,
        #[command(subcommand)]
        command: IssuesSubCommands,
    },

    Test {},
}
#[derive(Subcommand)]
enum IssuesSubCommands {
    Find {
        #[arg(short, long)]
        search: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Commit { message } => {
            git::commit(message);
        }
        Commands::Repos { format, org } => delegations::gh::get_all_repos(org, format),
        Commands::Test {} => {}
        Commands::Issue {
            format,
            org,
            command,
        } => match command {
            IssuesSubCommands::Find { search } => {
                delegations::gh::find_issue_by_text(org, format, search)
            }
        },
    }
}
