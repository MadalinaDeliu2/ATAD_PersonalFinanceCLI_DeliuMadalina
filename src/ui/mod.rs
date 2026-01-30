mod app;

use crate::models::transaction::Transaction;
use crate::models::budget::Budget;
use crate::db::{load_transactions, load_budgets};
use app::{App, AppScreen};
use std::io;
use std::collections::{HashMap, BTreeMap};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use rusqlite::Connection;


pub fn run_tui(conn: &Connection) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let transactions = match load_transactions(conn) {
        Ok(list) => list,
        Err(err) => {
            println!("Failed to load transactions: {}", err);
            Vec::new()
        }
    };

    let budgets = match load_budgets(conn) {
        Ok(list) => list,
        Err(err) => {
            println!("Failed to load budgets: {}", err);
            Vec::new()
        }
    };

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            match app.screen {
                AppScreen::MainMenu => draw_main_menu(f, &app),
                AppScreen::Transactions => draw_transactions(f, &app, &transactions),
                AppScreen::Budgets => draw_budgets(f, &app, &budgets, &transactions),
                AppScreen::Reports => draw_reports_by_category(f, &app, &transactions),
            }
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('m') => app.screen = AppScreen::MainMenu,

                    KeyCode::Up => {
                        if app.menu_index > 0 {
                            app.menu_index -= 1;
                        }
                    }

                    KeyCode::Down => {
                        if app.menu_index < 4 {
                            app.menu_index += 1;
                        }
                    }

                    KeyCode::Enter => match app.menu_index {
                        0 => app.screen = AppScreen::Transactions,
                        1 => app.screen = AppScreen::Budgets,
                        2 => app.screen = AppScreen::Reports,
                        3 => break,
                        _ => {}
                    },

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}


use ratatui::widgets::{List, ListItem, Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::style::{Style, Color};


fn draw_main_menu(f: &mut ratatui::Frame, app: &App) {
    let menu_items = vec![
        "Transactions",
        "Budgets",
        "Reports",
        "Quit",
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            if i == app.menu_index {
                ListItem::new(format!("> {}", item))
                    .style(Style::default().fg(Color::Yellow).bg(Color::Blue))
            } else {
                ListItem::new(format!("  {}", item))
                    .style(Style::default().fg(Color::White))
            }
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Main Menu ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        );

    f.render_widget(list, f.size());
}


fn draw_transactions(
    f: &mut ratatui::Frame,
    _app: &App,
    transactions: &[Transaction],
) {
    let items: Vec<ListItem> = transactions
        .iter()
        .map(|t| {
            let text = format!(
                "{} | {} | {:.2} lei | {}",
                t.id,
                t.date,
                t.amount,
                t.description.clone().unwrap_or_default()
            );

            ListItem::new(text).style(Style::default().fg(Color::White))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Transactions ")
                .borders(Borders::ALL)
        );

    f.render_widget(list, f.size());
}


fn draw_budgets(
    f: &mut ratatui::Frame,
    _app: &App,
    budgets: &[Budget],
    transactions: &[Transaction],
) {
    let mut spent_per_category: HashMap<String, f64> = HashMap::new();

    for t in transactions {
        let cat = t
            .category
            .clone()
            .unwrap_or_else(|| "Uncategorized".to_string());
        if t.amount < 0.0 {
            *spent_per_category.entry(cat).or_insert(0.0) += -t.amount; 
        }
    }

    let mut items: Vec<ListItem> = Vec::new();

    for b in budgets {
        let spent = spent_per_category
            .get(&b.category)
            .cloned()
            .unwrap_or(0.0);
        let remaining = b.limit_amount - spent;
        let percent = if b.limit_amount > 0.0 {
            (spent / b.limit_amount * 100.0).min(999.9)
        } else {
            0.0
        };

        let text = format!(
            "{}: limit {:.2} lei | spent {:.2} lei | remaining {:.2} lei ({:.1}%)",
            b.category, b.limit_amount, spent, remaining, percent
        );

        let style = if remaining < 0.0 {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::White)
        };

        items.push(ListItem::new(text).style(style));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Budgets (limit + spent) ")
                .borders(Borders::ALL)
        );

    f.render_widget(list, f.size());
}


fn draw_reports_by_category(
    f: &mut ratatui::Frame,
    _app: &App,
    transactions: &[Transaction],
) {
    use ratatui::widgets::{Block, Borders, Paragraph};
    use ratatui::style::{Style, Color};
    use chrono::Local;
    use std::collections::BTreeMap;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.size());

    let now = Local::now();
    let month_name = now.format("%B").to_string();     // ex: "January"
    let month_numeric = now.format("%m/%Y").to_string(); // ex: "01/2026"

    let header = Block::default()
        .title(format!(
            " Reports by category — {} ({}) ",
            month_name, month_numeric
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));
    f.render_widget(header, chunks[0]);

    let mut per_category: BTreeMap<String, f64> = BTreeMap::new();

    for t in transactions {
        if t.amount >= 0.0 {
            continue;
        }

        if t.date.len() < 10 {
            continue;
        }

        let tx_month = format!("{}{}", &t.date[0..3], &t.date[6..10]); 
        if tx_month != month_numeric {
            continue; 
        }

        let cat = t
            .category
            .clone()
            .unwrap_or_else(|| "Uncategorized".to_string());

        *per_category.entry(cat).or_insert(0.0) += -t.amount;
    }

    if per_category.is_empty() {
        let p = Paragraph::new("No expenses found for the current month.")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Category trends ")
            )
            .style(Style::default().fg(Color::White));
        f.render_widget(p, chunks[1]);
        return;
    }

    let mut full_text = String::new();

    let max = per_category
        .values()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1.0);

    let max_bar_width = 30.0;

    for (cat, value) in &per_category {
        let bar_len = ((value / max) * max_bar_width).round() as usize;
        let bar = "█".repeat(bar_len.max(1));

        full_text.push_str(&format!(
            "{}  {:<30}  {:.2} lei\n",
            cat,
            bar,
            value
        ));
    }

    full_text.push_str("----------------------------------------\n");

    let chart = Paragraph::new(full_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Monthly expenses by category ")
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(chart, chunks[1]);
}
