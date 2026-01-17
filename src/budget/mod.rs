use rusqlite::Connection;

pub fn set_budget(conn: &Connection, category: &str, limit: f64) {
    conn.execute(
        "INSERT INTO budgets (category, limit_amount)
         VALUES (?1, ?2)
         ON CONFLICT(category) DO UPDATE SET limit_amount = excluded.limit_amount",
        (category, limit),
    ).expect("Failed to set budget");
	
	println!("Budget added: {} → {} lei", category, limit);
}



pub fn spent_this_month(conn: &Connection, category: &str) -> f64 {
    let mut stmt = conn.prepare(
        "SELECT SUM(amount) FROM transactions
         WHERE category = ?1
         AND substr(date, 1, 2) = strftime('%m', 'now')
         AND substr(date, -4) = strftime('%Y', 'now')"
    ).unwrap();

    stmt.query_row([category], |row| {
        Ok(row.get::<_, Option<f64>>(0)?.unwrap_or(0.0).abs())
    }).unwrap()
}


pub fn check_budget(conn: &Connection, category: &str) {
    let mut stmt = conn.prepare(
        "SELECT limit_amount FROM budgets WHERE category = ?1"
    ).unwrap();

    let limit: f64 = stmt.query_row([category], |row| row.get(0)).unwrap();
    let spent = spent_this_month(conn, category);

    println!("Category: {}", category);
    println!("Spent: {} lei", spent);
    println!("Limit: {} lei", limit);

    let percent = spent / limit;

    if percent >= 1.0 {
        println!("⚠️ ALERT: Budget exceeded!");
    } else if percent >= 0.8 {
        println!("Warning: You reached 80% of your budget.");
    } else {
        println!("✓ You are within the budget.");
    }
}

pub fn check_all(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT category FROM budgets").unwrap();
    let rows = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();

    for cat in rows {
        if let Ok(category) = cat {
            check_budget(conn, &category);
            println!("----------------------");
        }
    }
}
