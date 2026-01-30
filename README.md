# 📘 Personal Finance CLI Manager

A Rust‑based command‑line application for tracking expenses, income, budgets, and generating financial reports directly from the terminal.  
The project includes both a **CLI interface** and an **interactive TUI (Terminal UI)**.

---

##  Main Features

- Import transactions from CSV files  
- Add transactions manually  
- Search transactions by keyword  
- Create and update category budgets  
- Generate monthly financial reports  
- Interactive TUI with menu navigation  
- SQLite database persistence  
- ASCII bar charts for category spending    

---

##  Available CLI Commands

Below is a list of all supported commands with short descriptions.

### **`add`**
Adds a new transaction manually.

### **`search`**
Searches transactions by keyword (category).

### **`import`**
Imports transactions from a CSV file.

### **`budget`**
Creates or updates a budget for a specific category.

### **`reports`**
Generates financial reports (default: current month).

### **`tui`**
Opens the interactive Terminal UI.

### **`help`**
Displays all available commands.

---

##  Command Examples

###  Add a transaction
```bash
cargo run -- add --amount=-20 --category Transport --description Taxi --date 01/10/2026
```

###  Search transaction
```bash
cargo run -- search
```
#### Search transaction by category
```bash
cargo run -- search --keyword Transport
```

###  Import CSV
```bash
cargo run -- import -f data/Transactions.csv
```

###  Set a budget
```bash
cargo run -- budget --category Food --limit 1000
```

###  Generate reports
```bash
cargo run -- reports
```
####  Generate reports for a specific month
```bash
cargo run -- reports --month 12 --year 2025
```

###  Open the TUI
```bash
cargo run -- tui
```
---

## Terminal UI (TUI)

The TUI provides an interactive interface with a main menu containing four sections:

### **1. Transactions**
- Displays all imported or manually added transactions  
- Easy navigation through the list  
- Shows amount, category, description, and date  

### **2. Budgets**
- Displays all budgets  
- Shows how much has been spent vs. the limit  
- Highlights categories that exceed their budget  

### **3. Reports**
- Shows the monthly spending report  
- Includes proportional ASCII bar charts  
- Automatically filters by the current month  

### **4. Quit**
- Exits the TUI  

### **Navigation**
- `m` → return to main menu  
- Arrow keys → navigate lists  
- `q` → quit  

<img width="1302" height="300" alt="image" src="https://github.com/user-attachments/assets/234f849c-a00b-4013-b4b9-4db362f01da1" />
<img width="1306" height="366" alt="image" src="https://github.com/user-attachments/assets/6a473299-ef29-43be-9094-2090df1a1cee" />
<img width="1306" height="347" alt="image" src="https://github.com/user-attachments/assets/193950bb-e6d3-49c2-95a2-b7a2801249f8" />
<img width="1306" height="224" alt="image" src="https://github.com/user-attachments/assets/0d75bf8e-d47f-4054-b445-f8a93b86609d" />
<img width="1327" height="280" alt="image" src="https://github.com/user-attachments/assets/444466c8-5c8a-4d89-a0e3-54e4d1a66b22" />


---

## CSV Format 
The CSV file must contain the following columns: amount, category, description, date.

### Example:
- -50,Food,McDonalds,01/09/2026
- -350,Food,Lidl,01/08/2026
- 2000,Salary,Company,01/01/2026


### Rules:
- Negative amounts = expenses  
- Positive amounts = income  
- Date format must be **MM/DD/YYYY**  
- Category and description are free text  


