use crate::screens::lib::{Action, AddProblemScreen, View};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::Modifier;
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
use ratatui::widgets::Borders;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, Paragraph};
use std::io;
use tui_textarea::TextArea;

impl<'a> AddProblemScreen<'a> {
    /// This is your "constructor"
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        let mut problem_area = TextArea::default();
        // Setup the textarea appearance ONCE here
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Name of Problem "),
        );

        textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));

        problem_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Problem Rating "),
        );

        problem_area.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));

        Self {
            textarea,
            problem_area,
        }
    }
}

impl<'a> View for AddProblemScreen<'a> {
    fn handle_events(&mut self) -> io::Result<Action> {
        let mut some_action = Action::NoOp;
        let current_event = event::read()?;

        match current_event {
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
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Action {
        match key_event.code {
            KeyCode::Esc => Action::ShouldSwitch,
            _ => {
                self.textarea.input(key_event);
                Action::NoOp
            }
        }
    }
}

impl<'a> Widget for &AddProblemScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Back ".into(),
            Span::styled(
                "<ESC>",
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
        ]);
        let container_block = Block::default()
            .borders(Borders::ALL)
            .title_top(" 💻 Add Problem with Rating ")
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        container_block.clone().render(area, buf);

        let inner_area = container_block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(inner_area);

        let input_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]); // We split the middle vertical chunk
        //

        let header_text = "Here you can add a problem with the rating: 1-5";
        Paragraph::new(header_text)
            .centered()
            .style(Style::default().fg(Color::Red))
            .render(chunks[0], buf);

        // Rendering the textarea section without styles
        self.textarea.render(input_chunks[0], buf);
        self.problem_area.render(input_chunks[1], buf)
    }
}
