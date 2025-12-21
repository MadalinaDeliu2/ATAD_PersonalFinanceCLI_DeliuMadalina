use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct ParsedTransaction {
    pub amount: f64,
    pub category: String,
    pub date: String,
}

pub fn parse_csv(file_path: &str) -> Vec<ParsedTransaction> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader.lines()
        .skip(1)
        .filter_map(|line| {
            if let Ok(entry) = line {
                let parts: Vec<&str> = entry.split(',').collect();
                if parts.len() >= 3 {
                    let amount = parts[0].trim().parse().ok()?;
                    let category = parts[1].trim().to_string();
                    let date = parts[2].trim().to_string();
                    Some(ParsedTransaction { amount, category, date })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}
