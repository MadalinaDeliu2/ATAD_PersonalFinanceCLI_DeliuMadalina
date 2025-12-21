mod cli;
mod db;
mod parser;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    let conn = db::init_db();

    match cli.command {
    cli::Commands::Add { amount, category, date } => {
        db::add_transaction(&conn, amount, &category, date);
    }
    cli::Commands::Search { keyword } => {
        db::search_transactions(&conn, keyword).unwrap();
    }
	cli::Commands::Import { file, r#type } => {
    db::import_transactions(&conn, &file, &r#type);
    }
    _ => println!("Feature not implemented yet"),
    }

}
