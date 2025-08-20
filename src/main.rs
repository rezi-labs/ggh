use clap::{Parser, Subcommand, ValueEnum, command};

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
    #[command(about = "git extensions")]
    Git {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(about = "gh extensions")]
    Gh {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(about = "curl exetensions")]
    Curl {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
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
        Commands::Git { args } => {
            delegations::git::run(args);
        }
        Commands::Gh { args } => delegations::gh::run(args),
        Commands::Curl { args } => {
            delegations::curl::run(args);
        }
        Commands::Repos { format, org } => delegations::gh::get_all_repos(org, format),
    }
}
