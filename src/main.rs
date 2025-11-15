use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "finance-cli")]
#[command(about = "Track income and expenses from the terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        amount: f64,

        #[arg(short, long)]
        category: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { amount, category } => {
            println!("Added transaction: {} in category '{}'", amount, category);
        }
    }
}
