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
use ratatui::widgets::Borders;
use ratatui::widgets::Padding;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, Paragraph};
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
        let successful_problem_added = false;
        let failed_to_add_problem = false;
        let sucessfully_updated_problem = false;
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
            successful_problem_added,
            failed_to_add_problem,
            sucessfully_updated_problem,
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
            KeyCode::Enter => {
                self.confirm_popup = !self.confirm_popup;
                Action::NoOp
            }
            KeyCode::Esc => {
                self.confirm_popup = false;
                Action::ShouldSwitch
            }
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
                        KeyCode::Char('A') => {
                            let problem_name: String = self.problem_name.lines().join("\n");

                            let problem_rating: String = self.problem_area.lines().join("\n");

                            match insert_new_problem(&self.db, &problem_name, &problem_rating) {
                                Ok(true) => {
                                    self.successful_problem_added = true;
                                }
                                Ok(false) => {
                                    self.failed_to_add_problem = true;
                                }
                                Err(error) => {
                                    println!("There was an error adding problem: {:#?}", error);
                                }
                            }
                        }
                        KeyCode::Char('U') => {
                            let problem_name: String = self.problem_name.lines().join("\n");

                            let problem_rating: String = self.problem_area.lines().join("\n");

                            match update_problem(&self.db, &problem_name, &problem_rating) {
                                Ok(true) => {
                                    self.sucessfully_updated_problem = true;
                                }
                                Ok(false) => {
                                    self.failed_to_add_problem = true;
                                }
                                Err(error) => {
                                    println!("There was an error adding problem: {:#?}", error);
                                }
                            }
                        }
                        KeyCode::Esc => {
                            self.confirm_popup = false;
                            self.successful_problem_added = false;
                            self.failed_to_add_problem = false;
                            self.sucessfully_updated_problem = false;
                        }
                        _ => {
                            self.confirm_popup = false;
                            self.successful_problem_added = false;
                            self.failed_to_add_problem = false;
                            self.sucessfully_updated_problem = false;
                        }
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
        if !self.confirm_popup {
            let instructions = Line::from(vec![
                " Back ".into(),
                Span::styled(
                    "<ESC>",
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
                .title_top(" 💻 Add / Update Problem with Rating ")
                .title_bottom(instructions.centered())
                .border_set(border::THICK)
                .padding(Padding::new(1, 1, 2, 2));

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

            let header_text = "Here you can add / update a problem with the rating: 1-5";
            Paragraph::new(header_text)
                .centered()
                .style(Style::default().fg(Color::Red))
                .render(chunks[0], buf);

            self.problem_name.render(input_chunks[0], buf);
            self.problem_area.render(input_chunks[1], buf);
        } else {
            let instructions = Line::from(vec![
                " Back ".into(),
                Span::styled(
                    "<ESC>",
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
                " Add ".into(),
                Span::styled(
                    "<A>",
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
                " Update ".into(),
                Span::styled(
                    "<U> ",
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
            ]);
            let inner_area = popup_area(area, 60, 20);

            Block::default()
                .borders(Borders::ALL)
                .title_top("Add / Update Confirmation ")
                .title_bottom(instructions.centered())
                .border_set(border::THICK)
                .render(inner_area, buf);

            let inner_popup_area = popup_area(inner_area, 60, 20);
            if self.successful_problem_added {
                Paragraph::new("Added new problem to db")
                    .centered()
                    .style(Style::default().fg(Color::LightGreen))
                    .render(inner_popup_area, buf);
            } else if self.failed_to_add_problem {
                Paragraph::new("Could not add problem")
                    .centered()
                    .style(Style::default().fg(Color::LightRed))
                    .render(inner_popup_area, buf);
            } else if self.sucessfully_updated_problem {
                Paragraph::new("Successfully Updated Problem")
                    .centered()
                    .style(Style::default().fg(Color::LightCyan))
                    .render(inner_popup_area, buf);
            }
        }
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

fn check_row_exists(db: &Arc<rusqlite::Connection>, problem_name: &str) -> rusqlite::Result<bool> {
    let query = "SELECT EXISTS(SELECT 1 FROM user_problems WHERE problem_name = ?1)";
    let exists = db.query_row(query, [problem_name], |row| row.get(0))?;

    Ok(exists)
}

fn update_problem(
    db: &Arc<rusqlite::Connection>,
    problem_name: &String,
    problem_rating: &String,
) -> rusqlite::Result<bool> {
    // Changed return type
    let table_name = "user_problems";

    // Ensure table exists
    if !check_table(db, table_name).unwrap_or(false) {
        db.execute(
            "CREATE TABLE IF NOT EXISTS user_problems (
                problem_name TEXT NOT NULL,
                problem_rating TEXT NOT NULL
            )",
            (),
        )?;
    }

    if check_row_exists(db, problem_name).unwrap() {
        db.execute(
            "UPDATE user_problems SET problem_name = ?1 WHERE problem_rating = ?2",
            (problem_name, problem_rating),
        )?;
        return Ok(true);
    }

    Ok(false)
}

fn insert_new_problem(
    db: &Arc<rusqlite::Connection>,
    problem_name: &String,
    problem_rating: &String,
) -> rusqlite::Result<bool> {
    // Changed return type
    let table_name = "user_problems";

    // Ensure table exists
    if !check_table(db, table_name).unwrap_or(false) {
        db.execute(
            "CREATE TABLE IF NOT EXISTS user_problems (
                problem_name TEXT NOT NULL,
                problem_rating TEXT NOT NULL
            )",
            (),
        )?;
    }

    // Check if row exists
    if !check_row_exists(db, problem_name).unwrap_or(false) {
        db.execute(
            "INSERT INTO user_problems (problem_name, problem_rating) VALUES (?1, ?2)",
            (problem_name, problem_rating),
        )?;
        return Ok(true); // Signifies a new row was added
    }

    Ok(false) // Signifies nothing was added, but no error occurred
}
