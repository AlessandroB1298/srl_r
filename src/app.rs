use crate::io;
use crate::screens::lib::{
    Action, AddProblemScreen, HomeScreen, MenuScreen, Screen, ScreenAction, View,
    ViewAllProblemsScreen,
};
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use std::sync::Arc;

#[derive(Debug)]
pub struct App<'a> {
    pub should_quit: bool,
    pub current_screen: Screen<'a>,
    pub db: Arc<rusqlite::Connection>, // Shared ownership
}

impl<'a> App<'a> {
    pub fn new(db: Arc<rusqlite::Connection>) -> Self {
        Self {
            db,
            current_screen: Screen::HomeScreen(HomeScreen::default()),
            should_quit: false,
        }
    }
    pub fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()?
            && key_event.kind == KeyEventKind::Press
        {
            let view: &mut dyn View = match &mut self.current_screen {
                Screen::HomeScreen(home) => home,
                Screen::MenuScreen(second) => second,
                Screen::AddProblemScreen(add) => add,
                Screen::ViewAllProblemsScreen(problem_screen) => problem_screen,
            };

            let action = view.handle_key_event(key_event);

            match action {
                Action::Quit => self.quit(),
                Action::ShouldSwitch => self.switch_screens(),

                // The magic happens here: process screen-specific actions
                Action::ScreenSpecific(screen_action) => {
                    self.handle_screen_action(screen_action);
                }

                Action::NoOp => {}
            }
        };
        Ok(())
    }

    fn handle_screen_action(&mut self, action: ScreenAction) {
        if let Screen::MenuScreen(_second) = &mut self.current_screen {
            match action {
                ScreenAction::MenuNext => self.move_menu_selection(1),
                ScreenAction::MenuPrev => self.move_menu_selection(-1),
                ScreenAction::MenuSelect => self.select_menu_item(),
            }
        } else if let Screen::ViewAllProblemsScreen(_problem_screen) = &mut self.current_screen {
            match action {
                ScreenAction::MenuNext => self.move_menu_selection(1),
                ScreenAction::MenuPrev => self.move_menu_selection(-1),
                ScreenAction::MenuSelect => self.select_menu_item(),
            }
        }
    }

    // NEW helper function to update menu selection
    fn move_menu_selection(&mut self, direction: isize) {
        if let Screen::MenuScreen(second) = &mut self.current_screen {
            let i = match second.menu_state.selected() {
                Some(i) => {
                    let len = second.menu_options.len();
                    (i as isize + direction).rem_euclid(len as isize) as usize
                }
                None => 0,
            };
            second.menu_state.select(Some(i));
        } else if let Screen::ViewAllProblemsScreen(problem_screen) = &mut self.current_screen {
            let i = match problem_screen.list_state.selected() {
                Some(i) => {
                    let len = problem_screen.items.len();
                    (i as isize + direction).rem_euclid(len as isize) as usize
                }
                None => 0,
            };
            problem_screen.list_state.select(Some(i));
        }
    }

    // NEW helper function to handle selection (e.g., switch screen based on menu option)
    fn select_menu_item(&mut self) {
        if let Screen::MenuScreen(second) = &mut self.current_screen
            && let Some(selected) = second.menu_state.selected()
        {
            match selected {
                0 => self.switch_screen_menu(0),
                1 => self.switch_screen_menu(1),
                2 => self.switch_screen_menu(2),
                3 => self.switch_screen_menu(3),
                _ => {}
            }
        }
    }

    pub fn switch_screen_menu(&mut self, index: i8) {
        match index {
            0 => {
                self.current_screen =
                    Screen::AddProblemScreen(AddProblemScreen::new(Arc::clone(&self.db)))
            }
            1 => {
                self.current_screen =
                    Screen::ViewAllProblemsScreen(ViewAllProblemsScreen::new(Arc::clone(&self.db)))
            }
            _ => {}
        }
    }

    pub fn switch_screens(&mut self) {
        match self.current_screen {
            Screen::HomeScreen(_) => {
                self.current_screen = Screen::MenuScreen(MenuScreen::default());
            }
            Screen::MenuScreen(_) => {
                self.current_screen = Screen::HomeScreen(HomeScreen::default());
            }
            Screen::AddProblemScreen(_) => {
                self.current_screen = Screen::MenuScreen(MenuScreen::default());
            }
            Screen::ViewAllProblemsScreen(_) => {
                self.current_screen = Screen::MenuScreen(MenuScreen::default());
            }
        }
    }
    pub fn quit(&mut self) {
        self.should_quit = true
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| {
                self.draw(frame);
            })?;

            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        let view: &dyn View = match &self.current_screen {
            Screen::HomeScreen(home) => home,
            Screen::MenuScreen(second) => second,
            Screen::AddProblemScreen(add) => add,
            Screen::ViewAllProblemsScreen(problem_screen) => problem_screen,
        };

        view.draw(frame);
    }
}
