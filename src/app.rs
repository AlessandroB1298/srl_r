use crate::components::screens;
use crate::components::screens::Action;
use crate::components::screens::HomeScreen;
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
    pub fn handle_events(&mut self) -> io::Result<Action> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                let view: &mut dyn View = match &mut self.current_screen {
                    screens::Screen::HomeScreen(home) => home,
                    screens::Screen::SecondScreen(second) => second,
                };

                let action = view.handle_key_event(key_event);
                match action {
                    Action::Quit => self.quit(),
                    Action::Default => {}
                    Action::NoOp => {}
                    Action::ShouldSwitch => {
                        self.switch_screens();
                    }
                }
            };
        }
        Ok(Action::Default)
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
            // Draw the screen
            terminal.draw(|frame| {
                self.draw(frame);
            })?;

            // Handle key presses
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
