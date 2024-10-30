mod budget_manager;

use budget_manager::{BudgetManager, TransactionType};
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "Budget Manager")]
#[command(version = "1.0")]
#[command(author = "Your Name")]
#[command(about = "A simple budget management tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new income transaction
    AddIncome {
        /// Amount of income
        amount: f64,
        /// Description of the income
        description: String,
    },
    /// Add a new expense transaction
    AddExpense {
        /// Amount of expense
        amount: f64,
        /// Description of the expense
        description: String,
    },
    /// View the current balance
    Balance,
    /// List all transactions
    List,
}

fn main() {
    let args = Cli::parse();

    let mut manager = BudgetManager::new();

    if let Err(e) = manager.load_from_file("transactions.json") {
        eprintln!("Warning: Could not load data: {}", e);
    }

    match args.command {
        Commands::AddIncome { amount, description } => {
            manager.add_transaction(amount, description, TransactionType::Income);
            println!("Income transaction added.");
        }
        Commands::AddExpense { amount, description } => {
            manager.add_transaction(amount, description, TransactionType::Expense);
            println!("Expense transaction added.");
        }
        Commands::Balance => {
            let balance = manager.get_balance();
            println!("Current Balance: {:.2}", balance);
        }
        Commands::List => {
            manager.list_transactions();
        }
    }

    if let Err(e) = manager.save_to_file("transactions.json") {
        eprintln!("Error saving data: {}", e);
        process::exit(1);
    }
}
