use crate::io;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::widgets::ListState;
use tui_textarea::TextArea;

#[derive(Debug)]
pub enum Screen<'a> {
    HomeScreen(HomeScreen),
    MenuScreen(MenuScreen),
    AddProblemScreen(AddProblemScreen<'a>),
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
pub struct AddProblemScreen<'a> {
    pub textarea: TextArea<'a>,
    pub problem_area: TextArea<'a>,
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
