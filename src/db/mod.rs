use chrono::Local;
use rusqlite::{params, Connection, Result};

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
