use crate::lib::{Action, MenuScreen, ScreenAction, View};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::Modifier;
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::text::Text;
use ratatui::text::{Line, Span};
use ratatui::widgets::Borders;
use ratatui::widgets::{Block, List, ListState, Paragraph};
use ratatui::widgets::{ListItem, Widget};
use std::io;

impl Default for MenuScreen {
    fn default() -> Self {
        let mut menu_state = ListState::default();
        // Set the initial selection to the first item (0)
        menu_state.select(Some(0));
        Self {
            menu_state,
            menu_options: &[
                "1. Add / Update Problem",
                "2. List All Problems ",
                "3. See Graph of Problems ",
            ],
        }
    }
}

impl View for MenuScreen {
    fn handle_events(&mut self) -> io::Result<Action> {
        let mut some_action = Action::NoOp;
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let result = self.handle_key_event(key_event);
                match result {
                    Action::Quit | Action::ShouldSwitch | Action::ScreenSpecific(_) => {
                        some_action = result;
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(some_action)
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Action {
        match key_event.code {
            KeyCode::Char('q') => Action::Quit, // Global
            KeyCode::Esc => Action::ShouldSwitch,

            // Context-specific actions are wrapped in ScreenSpecific
            KeyCode::Down => Action::ScreenSpecific(ScreenAction::MenuNext),
            KeyCode::Up => Action::ScreenSpecific(ScreenAction::MenuPrev),
            KeyCode::Enter => Action::ScreenSpecific(ScreenAction::MenuSelect),
            _ => Action::NoOp,
        }
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &MenuScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // --- 1. Instructions Block (Bottom Title) ---

        let instructions = Line::from(vec![
            " Quit ".into(),
            Span::styled(
                "<Q>",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            " Scroll ".into(),
            Span::styled(
                "<↑/↓>",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            " Enter".into(),
            Span::styled(
                "<Enter>",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            " ESC ".into(),
            Span::styled(
                "<ESC>",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let container_block = Block::default()
            .borders(Borders::ALL)
            .title_top(" 💻 SRL-Rust Menu ")
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        container_block.clone().render(area, buf);

        let inner_area = container_block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(0),
            ])
            .split(inner_area);

        let welcome_area = chunks[0];
        let menu_area = chunks[1];

        let welcome_text = Text::from(vec![Line::from(Span::styled(
            "Welcome to SRL-Rust Problem Tracker!",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))])
        .centered();

        Paragraph::new(welcome_text)
            .alignment(ratatui::layout::Alignment::Center)
            .render(welcome_area, buf);

        let selection_style = Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD);

        let items: Vec<ListItem> = self
            .menu_options
            .iter()
            .map(|&s| ListItem::new(s).style(Style::default().fg(Color::White)))
            .collect();

        let menu_list = List::new(items)
            .block(
                Block::default()
                    .title(" Main Options ")
                    .borders(Borders::NONE),
            )
            .highlight_style(selection_style)
            .highlight_symbol(">> ");

        let mut temp_state = self.menu_state.clone();

        ratatui::widgets::StatefulWidget::render(menu_list, menu_area, buf, &mut temp_state);
    }
}
