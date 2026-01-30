#[derive(Debug, Clone, Copy)]
pub enum AppScreen {
    MainMenu,
    Transactions,
    Budgets,
    Reports,
}

pub struct App {
    pub screen: AppScreen,
    pub menu_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: AppScreen::MainMenu,
            menu_index: 0,
        }
    }
}
