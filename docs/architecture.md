# Architecture Document – Personal Finance CLI Manager

## Overview
The Personal Finance CLI Manager is a Rust-based command-line application for tracking income and expenses.  
It is designed around modular components to ensure clarity, maintainability, and extensibility.

---

## High-Level Design
The system is divided into several core modules:

- **CLI Layer (`cli.rs`)**
  - Parses user input and subcommands (`add`, `import`, `report`, `budget`, `search`).
  - Delegates execution to the appropriate service module.

- **Database Layer (`db.rs`)**
  - Handles persistence using SQLite.
  - Provides CRUD operations for transactions, categories, and budgets.

- **Transactions Module (`transactions.rs`)**
  - Imports transactions from CSV/OFX files.
  - Allows manual addition of income and expenses.
  - Applies categorization rules (regex-based).

- **Budget Module (`budget.rs`)**
  - Stores budget limits per category.
  - Generates alerts when spending exceeds thresholds.

- **Reports Module (`reports.rs`)**
  - Produces summaries (monthly spending, category breakdown).
  - Displays charts and tables directly in the terminal.

---

## Data Flow
1. **User Input** → CLI parses subcommand and arguments.  
2. **Command Execution** → CLI calls the corresponding module.  
3. **Database Interaction** → Data is stored/retrieved from SQLite.  
4. **Output** → Results are displayed in the terminal (text or charts).

