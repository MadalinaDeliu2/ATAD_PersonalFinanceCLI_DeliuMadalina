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
        #[arg(short = 'a', long)]
        amount: f64,
        #[arg(short = 'c', long)]
        category: String,
        #[arg(short = 'd', long)]
		description: String,
        #[arg(short = 'D', long)]
        date: Option<String>,
    },

    Import {
        #[arg(short, long)]
        file: String,
        #[arg(short, long, default_value = "csv")]
        r#type: String,
    },

    Reports {
    #[arg(long)]
    month: Option<String>,   
    #[arg(long)]
    year: Option<String>,   
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
	Sql { #[arg(short, long)] query: String, },
	Tui,

}
