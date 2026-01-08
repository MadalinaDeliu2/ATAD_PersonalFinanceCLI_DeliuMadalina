use rusqlite::Connection;
use regex::Regex;


fn get_rules() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new("uber|taxi|transport").unwrap(), "Transport"),
        (Regex::new("kaufland|carrefour|lidl|food").unwrap(), "Food"),
        (Regex::new("netflix|spotify|youtube").unwrap(), "Entertainment"),
        (Regex::new("rent|chirie").unwrap(), "Housing"),
        (Regex::new("salary|income|transfer").unwrap(), "IncomingTransfer"),
    ]
}


pub fn categorize_transactions(conn: &Connection) {

    let mut stmt = conn
        .prepare(
            "SELECT id, description 
             FROM transactions 
             WHERE category IS NULL OR category = ''"
        )
        .expect("Failed to prepare query");

    let rows = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            let description: String = row.get(1)?;
            Ok((id, description))
        })
        .expect("Failed to query uncategorized transactions");

    let rules = get_rules();
    let mut updated = 0;

    for result in rows {
        if let Ok((id, description)) = result {
            let desc_lower = description.to_lowercase();

            for (regex, new_category) in &rules {
                if regex.is_match(&desc_lower) {
                    conn.execute(
                        "UPDATE transactions SET category = ?1 WHERE id = ?2",
                        (new_category, id),
                    )
                    .expect("Failed to update category");

                    updated += 1;
                    break;
                }
            }
        }
    }

    println!("Categorization complete. {} transactions updated.", updated);
}
