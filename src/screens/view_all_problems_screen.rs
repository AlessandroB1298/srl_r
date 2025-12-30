use crate::screens::lib::{Action, Problem, ScreenAction, View, ViewAllProblemsScreen};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::Modifier;
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::text::Text;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Cell, Table, TableState};
use ratatui::widgets::{Borders, Paragraph};
use ratatui::widgets::{Row, Widget};
use std::io;
use std::sync::Arc;

impl<'a> ViewAllProblemsScreen<'a> {
    pub fn new(db: Arc<rusqlite::Connection>) -> Self {
        let mut list_state = TableState::default();
        list_state.select(Some(0));
        let items = query_items(&db).unwrap();
        Self {
            db,
            items,
            list_state,
        }
    }
}

fn query_items(db: &Arc<rusqlite::Connection>) -> rusqlite::Result<Vec<Row<'static>>> {
    let mut db_result = db.prepare("SELECT * FROM user_problems")?;
    let problem_iter = db_result.query_map([], |row| {
        Ok(Problem {
            name: row.get(0)?,
            rating: row.get(1)?,
            entry_date: row.get(2)?,
        })
    })?;
    let mut items: Vec<Row> = vec![];
    for problem_result in problem_iter {
        let problem = problem_result?;
        let cells = vec![
            Cell::from(problem.name),
            Cell::from(problem.rating),
            Cell::from(problem.entry_date),
        ];
        let row = Row::new(cells).height(2);
        items.push(row);
    }

    Ok(items)
}

impl<'a> View for ViewAllProblemsScreen<'a> {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
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

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Action {
        match key_event.code {
            KeyCode::Char('q') => Action::Quit, // Global
            KeyCode::Esc => Action::ShouldSwitch,

            KeyCode::Down => Action::ScreenSpecific(ScreenAction::MenuNext),
            KeyCode::Up => Action::ScreenSpecific(ScreenAction::MenuPrev),
            KeyCode::Enter => Action::ScreenSpecific(ScreenAction::MenuSelect),
            _ => Action::NoOp,
        }
    }
}

impl<'a> Widget for &ViewAllProblemsScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
            .border_set(border::THICK)
            .border_style(Style::default().fg(Color::Cyan))
            .title_top(Line::from(" 🔎 Problem Database Explorer ").centered())
            .title_bottom(instructions.centered());
        container_block.clone().render(area, buf);

        let inner_area = container_block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
            ])
            .split(inner_area);

        let welcome_area = chunks[0];
        //let menu_area = chunks[1];

        let welcome_text = Text::from(vec![Line::from(Span::styled(
            "View all problems added to database!",
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

        let widths = [
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ];
        let rows = self.items.clone();

        let table = Table::new(rows, widths)
            .header(Row::new(vec!["Problem Name", "Problem Rating", "Last Entry"]).bottom_margin(1))
            .column_spacing(20)
            .row_highlight_style(selection_style)
            .highlight_symbol(">>");

        let mut temp_state = self.list_state.clone();

        ratatui::widgets::StatefulWidget::render(table, chunks[1], buf, &mut temp_state);
    }
}
