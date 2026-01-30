use rusqlite::Connection;

pub fn monthly_spending(conn: &Connection, month: &str, year: &str) -> f64 {
    let mut stmt = conn.prepare(
        "SELECT SUM(amount) FROM transactions
         WHERE substr(date, 1, 2) = ?1
         AND substr(date, -4) = ?2"
    ).unwrap();

    stmt.query_row([month, year], |row| {
        Ok(row.get::<_, Option<f64>>(0)?.unwrap_or(0.0).abs())
    }).unwrap()
}

pub fn category_breakdown(conn: &Connection, month: &str, year: &str) {
    let mut stmt = conn.prepare(
        "SELECT category, SUM(amount) FROM transactions
         WHERE substr(date, 1, 2) = ?1
         AND substr(date, -4) = ?2
         GROUP BY category"
    ).unwrap();

    let rows = stmt.query_map([month, year], |row| {
        let category: String = row.get(0)?;
        let total: f64 = row.get::<_, Option<f64>>(1)?.unwrap_or(0.0).abs();
        Ok((category, total))
    }).unwrap();

    println!("Category breakdown for {}/{}:", month, year);
    for row in rows {
        if let Ok((category, total)) = row {
            println!(" - {}: {} lei", category, total);
        }
    }
}
