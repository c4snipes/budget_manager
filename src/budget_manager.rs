use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: u32,
    pub date: DateTime<Local>,
    pub amount: f64,
    pub description: String,
    pub t_type: TransactionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
    Income,
    Expense,
}

pub struct BudgetManager {
    pub transactions: Vec<Transaction>,
    next_id: u32,
}

impl BudgetManager {
    pub fn new() -> Self {
        BudgetManager {
            transactions: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_transaction(
        &mut self,
        amount: f64,
        description: String,
        t_type: TransactionType,
    ) {
        let transaction = Transaction {
            id: self.next_id,
            date: Local::now(),
            amount,
            description,
            t_type,
        };
        self.transactions.push(transaction);
        self.next_id += 1;
    }

    pub fn get_balance(&self) -> f64 {
        self.transactions.iter().fold(0.0, |acc, trans| match trans.t_type {
            TransactionType::Income => acc + trans.amount,
            TransactionType::Expense => acc - trans.amount,
        })
    }

    pub fn list_transactions(&self) {
        if self.transactions.is_empty() {
            println!("No transactions found.");
            return;
        }

        for trans in &self.transactions {
            println!(
                "ID: {}, Date: {}, Type: {:?}, Amount: {:.2}, Description: {}",
                trans.id,
                trans.date.format("%Y-%m-%d %H:%M:%S"),
                trans.t_type,
                trans.amount,
                trans.description
            );
        }
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string_pretty(&self.transactions)?;
        std::fs::write(filename, serialized)?;
        Ok(())
    }

    pub fn load_from_file(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(contents) = std::fs::read_to_string(filename) {
            self.transactions = serde_json::from_str(&contents)?;
            self.next_id = self.transactions.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        }
        Ok(())
    }
}
