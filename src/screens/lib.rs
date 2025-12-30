use std::sync::Arc;

use crate::io;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::widgets::{ListItem, ListState, Row, TableState};
use tui_textarea::TextArea;

#[derive(Debug)]
pub enum Screen<'a> {
    HomeScreen(HomeScreen),
    MenuScreen(MenuScreen),
    AddProblemScreen(AddProblemScreen<'a>),
    ViewAllProblemsScreen(ViewAllProblemsScreen<'a>),
}

impl<'a> Default for Screen<'a> {
    fn default() -> Self {
        // We set the starting screen as the default
        Screen::HomeScreen(HomeScreen::default())
    }
}
#[derive(Debug, Default)]
pub struct HomeScreen {}

#[derive(Debug)]
pub struct MenuScreen {
    pub menu_state: ListState,
    pub menu_options: &'static [&'static str],
}

#[derive(Debug, Default)]
pub enum InputSelector {
    #[default]
    ProblemName,
    ProblemRating,
}

#[derive(Debug)]
pub struct AddProblemScreen<'a> {
    pub problem_name: TextArea<'a>,
    pub problem_rating: TextArea<'a>,
    pub entry_date: String,
    pub input_mode: InputSelector,
    pub db: Arc<rusqlite::Connection>,
    pub confirm_popup: bool,
    pub successful_problem_added: bool,
    pub failed_to_add_problem: bool,
    pub sucessfully_updated_problem: bool,
    pub incorrect_rating: bool,
    pub incorrect_name: bool,
}

#[derive(Debug)]
pub struct Problem {
    pub name: String,
    pub rating: String,
    pub entry_date: String,
}

#[derive(Debug)]
pub struct ViewAllProblemsScreen<'a> {
    pub db: Arc<rusqlite::Connection>,
    pub items: Vec<Row<'a>>,
    pub list_state: TableState,
}

pub enum Action {
    // Global Actions (Handled by App)
    Quit,
    ShouldSwitch,

    // Screen-Specific Actions (Handled by the current View)
    ScreenSpecific(ScreenAction), // <<< NEW: Holds context-specific actions

    // Fallback/No-Op
    NoOp,
}

pub enum ScreenAction {
    MenuNext,
    MenuPrev,
    MenuSelect,
}

pub trait View {
    fn handle_events(&mut self) -> io::Result<Action>;
    fn draw(&self, frame: &mut Frame);
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Action;
}
