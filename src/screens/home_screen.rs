use crate::lib::{Action, HomeScreen, View};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::text::Text;
use ratatui::widgets::Borders;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, Paragraph};

impl View for HomeScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Action {
        match key_event.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Enter => Action::ShouldSwitch,

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
                "Welcome to SRL-Rust, here we are implmeneting spaced repetiton learning in rust for leet code style questions",
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
