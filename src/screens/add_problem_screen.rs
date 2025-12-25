use crate::screens::lib::{Action, AddProblemScreen, InputSelector, View};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Flex, Rect};
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::Modifier;
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, Paragraph};
use ratatui::widgets::{Borders, Clear};
use std::io;
use std::sync::Arc;
use tui_textarea::TextArea;

impl<'a> AddProblemScreen<'a> {
    /// This is your "constructor"
    pub fn new(db: Arc<rusqlite::Connection>) -> Self {
        let mut problem_name = TextArea::default();
        let mut problem_area = TextArea::default();
        let input_mode = InputSelector::default();
        let confirm_popup = false;
        // Setup the textarea appearance ONCE here
        problem_name.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Name of Problem "),
        );

        problem_name.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));

        problem_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Rating 1-5 ")
                .style(Color::LightGreen),
        );

        problem_area.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));

        Self {
            problem_name,
            problem_area,
            input_mode,
            db,
            confirm_popup,
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
        if self.confirm_popup {
            let instructions = Line::from(vec![
                " No ".into(),
                Span::styled(
                    "<N>",
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
                " Yes".into(),
                Span::styled(
                    "<Y>",
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
            ]);
            let block = Block::default()
                .borders(Borders::ALL)
                .title_top("Confirm addition")
                .title_bottom(instructions.centered())
                .border_set(border::THICK);
            let area = popup_area(frame.area(), 60, 20);
            frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(block, area);
        } else {
            frame.render_widget(self, frame.area());
        }
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Action {
        match key_event.code {
            KeyCode::Enter => {
                self.confirm_popup = !self.confirm_popup;
                Action::NoOp
            }
            KeyCode::Esc => Action::ShouldSwitch,
            KeyCode::Tab => {
                // Switch focus on Tab press
                self.input_mode = match self.input_mode {
                    InputSelector::ProblemName => InputSelector::ProblemRating,
                    InputSelector::ProblemRating => InputSelector::ProblemName,
                };
                Action::NoOp
            }
            _ => {
                // to make sure we don't add to the inputs by accident
                if self.confirm_popup {
                    match key_event.code {
                        KeyCode::Char('Y') => {
                            //TODO: Add function to handle adding to sqlite db
                            // Use .lines() to get references instead of consuming the object
                            let problem_name: String = self.problem_name.lines().join("\n");

                            let problem_rating: String = self.problem_area.lines().join("\n");

                            insert_new_problem(&self.db, &problem_name, &problem_rating);
                        }
                        KeyCode::Char('N') => {
                            //TODO: Exit Confirm menu
                            self.confirm_popup = false;
                        }
                        _ => {}
                    }
                } else {
                    match self.input_mode {
                        InputSelector::ProblemName => {
                            self.problem_name.input(key_event);
                        }
                        InputSelector::ProblemRating => {
                            self.problem_area.input(key_event);
                        }
                    }
                }
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
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(inner_area);

        let input_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(chunks[1]); // We split the middle vertical chunk
        //

        let header_text = "Here you can add a problem with the rating: 1-5";
        Paragraph::new(header_text)
            .centered()
            .style(Style::default().fg(Color::Red))
            .render(chunks[0], buf);

        self.problem_name.render(input_chunks[0], buf);
        self.problem_area.render(input_chunks[1], buf);
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
#[derive(Debug)]
struct Problem {
    problem_name: String,
    problem_rating: String,
}
fn check_table(db: &Arc<rusqlite::Connection>, table_name: &str) -> rusqlite::Result<bool> {
    db.table_exists(None, table_name)
}

fn insert_new_problem(
    db: &Arc<rusqlite::Connection>,
    problem_name: &String,
    problem_rating: &String,
) -> rusqlite::Result<()> {
    let table_name = "user_problems";
    let table_exists = check_table(db, table_name);
    if !table_exists.unwrap() {
        db.execute(
            "CREATE TABLE IF NOT EXISTS user_problems (
            problem_name TEXT NOT NULL,
            problem_rating TEXT NOT NULL
        )",
            (),
        )?;
    }

    // 2. Insert the data
    db.execute(
        "INSERT INTO user_problems (problem_name, problem_rating) VALUES (?1, ?2)",
        (problem_name, problem_rating),
    )?;

    println!("Added to database successfully");

    Ok(())
}
