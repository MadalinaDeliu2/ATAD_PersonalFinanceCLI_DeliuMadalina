use chrono::Local;
use rusqlite::{params, Connection, Result};
use crate::models::transaction::Transaction;


pub fn init_db() -> Connection {
    let conn = Connection::open("finance.db").expect("Failed to open finance.db");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			amount REAL NOT NULL,
			category TEXT,
			description TEXT,
			date TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create transactions table");
	
	conn.execute( "CREATE TABLE IF NOT EXISTS budgets ( 
					category TEXT PRIMARY KEY, 
					limit_amount REAL NOT NULL 
					)", 
					[], 
	).expect("Failed to create budgets table");

    conn
}


pub fn add_transaction(conn: &Connection, amount: f64, category: &str, description: &str, date: Option<String>) {
    let date_str = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
    conn.execute(
        "INSERT INTO transactions (amount, category, description, date) VALUES (?1, ?2, ?3, ?4)",
        params![amount, category, description, date_str],
    ).expect("Failed to insert transaction");

    println!("Saved: {} in '{}' on {}", amount, category, date_str);
}





pub fn search_transactions(conn: &Connection, keyword: Option<String>) -> Result<()> {
    let (sql, params) = if let Some(k) = keyword {
        (
            "SELECT id, amount, category, description, date FROM transactions WHERE category LIKE ?1",
            params![format!("%{}%", k)],
        )
    } else {
        (
            "SELECT id, amount, category, description, date FROM transactions",
            params![],
        )
    };

    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(params, |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, String>(2)?,
			row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        ))
    })?;

    for row in rows {
        let (id, amount, category, description, date) = row?;
        println!("{} | {} | {} | {} | {}", id, amount, category, description, date);
    }

    Ok(())
}



pub fn import_transactions(conn: &Connection, file_path: &str, file_type: &str) {
    if file_type != "csv" {
        println!("Only CSV import is supported for now.");
        return;
    }

    let transactions = crate::parser::parse_csv(file_path);
    for tx in transactions {
        add_transaction(conn, tx.amount, &tx.category, &tx.description, Some(tx.date));
    }

    println!("Import completed.");
}

pub fn run_sql(conn: &Connection, query: &str) {
    let mut stmt = conn.prepare(query).expect("Invalid SQL");
    let column_count = stmt.column_count();

    let rows = stmt
        .query_map([], |row| {
            let mut values = Vec::new();
            for i in 0..column_count {
                let v: Result<String, _> = row.get(i);
                values.push(v.unwrap_or("NULL".into()));
            }
            Ok(values)
        })
        .expect("Query failed");

    for row in rows {
        println!("{:?}", row.unwrap());
    }
}


pub fn load_transactions(conn: &Connection) -> Result<Vec<Transaction>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, amount, category, description, date FROM transactions ORDER BY id ASC"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            amount: row.get(1)?,
            category: row.get(2)?,
            description: row.get(3)?,
            date: row.get(4)?,
        })
    })?;

    let mut transactions = Vec::new();
    for t in rows {
        transactions.push(t?);
    }

    Ok(transactions)
}

use crate::models::budget::Budget;

pub fn load_budgets(conn: &Connection) -> Result<Vec<Budget>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT category, limit_amount FROM budgets ORDER BY category ASC"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Budget {
            category: row.get(0)?,
            limit_amount: row.get(1)?,
        })
    })?;

    let mut budgets = Vec::new();
    for b in rows {
        budgets.push(b?);
    }

    Ok(budgets)
}




