use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "finance-cli")]
#[command(about = "Track income and expenses from the terminal")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        amount: f64,
        #[arg(short, long)]
        category: String,
        #[arg(short, long)]
        date: Option<String>,
    },

    Import {
        #[arg(short, long)]
        file: String,
        #[arg(short, long, default_value = "csv")]
        r#type: String,
    },

    Report {
        #[arg(short, long)]
        month: Option<String>,
    },

    Budget {
        #[arg(short, long)]
        category: Option<String>,
        #[arg(short, long)]
        limit: Option<f64>,
    },

    Search {
        #[arg(short, long)]
        keyword: Option<String>,
    },
}
