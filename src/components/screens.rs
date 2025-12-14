use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::style::{Color, Stylize};
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::text::Text;
use ratatui::widgets::Borders;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, Paragraph};
use std::io;

#[derive(Debug)]
pub enum Screen {
    HomeScreen(HomeScreen),
    SecondScreen(SecondScreen),
}
pub enum Action {
    Quit,
    ShouldSwitch,
    NoOp,
    Default,
}

impl Default for Screen {
    fn default() -> Self {
        // We set the starting screen as the default
        Screen::HomeScreen(HomeScreen::default())
    }
}

pub trait View {
    fn handle_events(&mut self) -> io::Result<Action>;
    fn draw(&self, frame: &mut Frame);
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Action;
}

#[derive(Debug, Default)]
pub struct HomeScreen {}

#[derive(Debug, Default)]
pub struct SecondScreen {}

impl View for HomeScreen {
    fn handle_events(&mut self) -> io::Result<Action> {
        let mut some_action = Action::Default;
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let result = self.handle_key_event(key_event);
                match result {
                    Action::Quit => {
                        some_action = result;
                    }
                    Action::NoOp => {}
                    Action::ShouldSwitch => {
                        some_action = result;
                    }
                    Action::Default => {}
                }
            }
            _ => {}
        };
        Ok(some_action)
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Action {
        match key_event.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Enter => Action::ShouldSwitch, // This now triggers the switch
            _ => Action::NoOp,
        }
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl View for SecondScreen {
    fn handle_events(&mut self) -> io::Result<Action> {
        let mut some_action = Action::Default;
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let result = self.handle_key_event(key_event);
                match result {
                    Action::Quit => {
                        some_action = result;
                    }
                    Action::NoOp => {}
                    Action::ShouldSwitch => {
                        some_action = result;
                    }
                    Action::Default => {}
                }
            }
            _ => {}
        };
        Ok(some_action)
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Action {
        match key_event.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Enter => Action::ShouldSwitch, // This now triggers the switch
            _ => Action::NoOp,
        }
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &HomeScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Enter ".into(),
            "<Enter> ".blue().bold(),
        ]);
        let container_block = Block::default()
            .borders(Borders::ALL)
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        container_block.clone().render(area, buf);

        let inner_area = container_block.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Length(10)])
            .split(inner_area);
        let banner_text = "

███████╗██████╗ ██╗         ██████╗ ██╗   ██╗███████╗████████╗
██╔════╝██╔══██╗██║         ██╔══██╗██║   ██║██╔════╝╚══██╔══╝
███████╗██████╔╝██║         ██████╔╝██║   ██║███████╗   ██║   
╚════██║██╔══██╗██║         ██╔══██╗██║   ██║╚════██║   ██║   
███████║██║  ██║███████╗    ██║  ██║╚██████╔╝███████║   ██║   
╚══════╝╚═╝  ╚═╝╚══════╝    ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   


        ";

        Paragraph::new(banner_text)
            .centered()
            .style(Style::default().fg(Color::Cyan))
            .render(chunks[0], buf);

        let welcome_text = Text::from(vec![
            Line::from(
                "Welcome to SRL-Rust, here we are implmeneting spaced repetitive learning in rust",
            ),
            Line::from("Press enter to begin..."),
        ])
        .centered();
        Paragraph::new(welcome_text)
            .centered()
            .style(Style::default().fg(Color::Yellow))
            .render(chunks[1], buf);
    }
}

impl Widget for &SecondScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Enter ".into(),
            "<Enter> ".blue().bold(),
        ]);
        let container_block = Block::default()
            .borders(Borders::ALL)
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        container_block.clone().render(area, buf);

        let inner_area = container_block.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Length(10)])
            .split(inner_area);

        let welcome_text = Text::from(vec![
            Line::from("Welcome to SRL-Rust, here is the second page"),
            Line::from("Press enter to begin..."),
        ])
        .centered();
        Paragraph::new(welcome_text)
            .centered()
            .style(Style::default().fg(Color::Yellow))
            .render(chunks[1], buf);
    }
}
