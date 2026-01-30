#[derive(Debug)]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub category: Option<String>,
    pub description: Option<String>,
    pub date: String,
}
