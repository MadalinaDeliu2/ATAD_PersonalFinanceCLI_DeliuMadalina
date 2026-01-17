mod cli;
mod db;
mod parser;
mod categorize;
mod budget;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    let conn = db::init_db();

    match cli.command {
    cli::Commands::Add { amount, category, description, date } => {
        db::add_transaction(&conn, amount, &category, &description, date);
    }
    cli::Commands::Search { keyword } => {
        db::search_transactions(&conn, keyword).unwrap();
    }
	cli::Commands::Import { file, r#type } => {
		db::import_transactions(&conn, &file, &r#type);
		categorize::categorize_transactions(&conn);
    }
	
	cli::Commands::Budget { category, limit } => {
		match (category, limit) {
			(Some(cat), Some(lim)) => budget::set_budget(&conn, &cat, lim),
			(Some(cat), None) => budget::check_budget(&conn, &cat),
			(None, None) => budget::check_all(&conn),
			_ => println!("Invalid usage."),
		}
		
	}
	
	cli::Commands::Sql { query } => {
    db::run_sql(&conn, &query);
}


    _ => println!("Feature not implemented yet"),
    }

}
