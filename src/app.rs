use crate::components::screens;
use crate::components::screens::Action;
use crate::components::screens::HomeScreen;
use crate::components::screens::ScreenAction;
use crate::components::screens::SecondScreen;
use crate::components::screens::View;
use crate::io;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::Frame;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub current_screen: screens::Screen,
}

impl App {
    pub fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                // 1. Get the action from the current screen
                let view: &mut dyn View = match &mut self.current_screen {
                    screens::Screen::HomeScreen(home) => home,
                    screens::Screen::SecondScreen(second) => second,
                };
                let action = view.handle_key_event(key_event);

                match action {
                    Action::Quit => self.quit(),
                    Action::ShouldSwitch => self.switch_screens(),

                    // The magic happens here: process screen-specific actions
                    Action::ScreenSpecific(screen_action) => {
                        self.handle_screen_action(screen_action);
                    }
                    Action::Default => {}

                    Action::NoOp => {}
                }
            };
        }
        Ok(())
    }

    fn handle_screen_action(&mut self, action: ScreenAction) {
        if let screens::Screen::SecondScreen(_second) = &mut self.current_screen {
            match action {
                ScreenAction::MenuNext => self.move_menu_selection(1),
                ScreenAction::MenuPrev => self.move_menu_selection(-1),
                ScreenAction::MenuSelect => self.select_menu_item(),

                _ => {}
            }
        }
    }

    // NEW helper function to update menu selection
    fn move_menu_selection(&mut self, direction: isize) {
        if let screens::Screen::SecondScreen(second) = &mut self.current_screen {
            let i = match second.menu_state.selected() {
                Some(i) => {
                    let len = second.menu_options.len();
                    let new_index = (i as isize + direction).rem_euclid(len as isize) as usize;
                    new_index
                }
                None => 0,
            };
            second.menu_state.select(Some(i));
        }
    }

    // NEW helper function to handle selection (e.g., switch screen based on menu option)
    fn select_menu_item(&mut self) {
        if let screens::Screen::SecondScreen(second) = &mut self.current_screen {
            if let Some(selected) = second.menu_state.selected() {
                // In a real app, you would switch to a new screen based on 'selected'
                // For now, let's just switch back to the home screen as an example
                match selected {
                    0 => println!("Action: Add New Problem"),
                    1 => println!("Action: Update Problem"),
                    2 => println!("Action: List All Problems"),
                    3 => println!("Action: See Graph of Problems"),
                    _ => {}
                }
                // self.switch_screens();
            }
        }
    }
    pub fn switch_screens(&mut self) {
        match self.current_screen {
            screens::Screen::HomeScreen(_) => {
                self.current_screen = screens::Screen::SecondScreen(SecondScreen::default());
            }
            screens::Screen::SecondScreen(_) => {
                self.current_screen = screens::Screen::HomeScreen(HomeScreen::default());
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
            screens::Screen::HomeScreen(home) => home,
            screens::Screen::SecondScreen(second) => second,
        };

        view.draw(frame);
    }
}
