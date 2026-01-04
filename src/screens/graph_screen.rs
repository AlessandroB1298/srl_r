use crate::lib::{Action, GraphScreen, Problem, View};
use chrono::{DateTime, Datelike, NaiveDate, ParseError, TimeZone, Utc, Weekday};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Row, Table, Widget};
use std::io;
use std::sync::Arc;

impl GraphScreen {
    pub fn new(db: Arc<rusqlite::Connection>) -> Self {
        let unprocessed_dates = query_items(&db);
        let dates = unprocessed_dates.unwrap();
        let date = Utc::now();
        let current_year = date.year() as usize;
        let offset = get_offset(current_year).unwrap() as i32;
        Self {
            db,
            dates,
            current_year,
            offset,
        }
    }
}
fn get_offset(year: usize) -> Result<u32, ParseError> {
    let day = NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap();

    let today_weekday = day.weekday();
    let index = today_weekday.days_since(Weekday::Mon);
    Ok(index)
}

fn query_items(db: &Arc<rusqlite::Connection>) -> rusqlite::Result<Vec<String>> {
    let mut db_result = db.prepare("SELECT * FROM user_problems")?;

    let problem_iter = db_result.query_map([], |row| {
        Ok(Problem {
            name: row.get(0)?,
            rating: row.get(1)?,
            entry_date: row.get(2)?,
        })
    })?;
    let mut items: Vec<String> = vec![];
    for problem_result in problem_iter {
        let problem = problem_result?;
        let problem_entry_date = utc_string_to_year_month_day(&problem.entry_date);
        match problem_entry_date {
            Ok(val) => {
                items.push(val);
            }
            Err(err) => {
                println!("error on 41: {:#?}", err);
            }
        }
    }

    Ok(items)
}
fn utc_string_to_year_month_day(utc_date_str: &str) -> Result<String, ParseError> {
    let datetime = utc_date_str.parse::<DateTime<Utc>>()?;

    // 2. Extract the year, month, and day components
    let year = datetime.year();
    let month = datetime.month(); // Month number starting from 1
    let day = datetime.day();
    let final_string = format!("{}/{}/{}", year, month, day);
    Ok(final_string)
}
fn to_day(date: &str) -> std::result::Result<u32, ParseError> {
    let format = "%Y/%m/%d";

    let naive_date = NaiveDate::parse_from_str(date.trim(), format)?;

    let ordinal_day = naive_date.ordinal();

    Ok(ordinal_day)
}

impl View for GraphScreen {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Action {
        match key_event.code {
            KeyCode::Esc => Action::ShouldSwitch,
            _ => Action::NoOp,
        }
    }
}

impl Widget for &GraphScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
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
            .title_top(Line::from(" 📈 Usage Graph ").centered())
            .title_bottom(instructions.centered());
        container_block.clone().render(area, buf);

        let inner_area = container_block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(0), // Area for the ■ grid
            ])
            .split(inner_area);

        let day_labels = vec![
            Row::new(vec!["Sun"]),
            Row::new(vec!["Mon"]),
            Row::new(vec!["Tue"]),
            Row::new(vec!["Wed"]),
            Row::new(vec!["Thu"]),
            Row::new(vec!["Fri"]),
            Row::new(vec!["Sat"]),
        ];
        // Use one constraint for the label column
        Table::new(day_labels, [Constraint::Length(5)]).render(chunks[0], buf);

        let mut dates: Vec<u32> = vec![];
        for date in &self.dates {
            let final_date = date.to_string();

            match to_day(&final_date) {
                Ok(val) => dates.push(val),
                Err(e) => {
                    eprintln!("Failed to parse date/time: {:?}", e);
                    return;
                }
            }
        }

        let rows: Vec<Row> = (0..7)
            .map(|day_of_week| {
                let cells: Vec<Cell> = (0..53)
                    .map(|week_index| {
                        let day_of_year = (week_index * 7) + (day_of_week) - self.offset + 1;

                        if day_of_year <= 0 || day_of_year > 365 {
                            return Cell::from("  ");
                        }
                        let count = dates.iter().filter(|&&d| d == day_of_year as u32).count();

                        match count {
                            0 => Cell::from("■").style(Style::default().fg(Color::DarkGray)),
                            1 => Cell::from("■")
                                .style(Style::default().fg(Color::Rgb(133, 199, 140))), // Green
                            2 => {
                                Cell::from("■").style(Style::default().fg(Color::Rgb(45, 230, 67)))
                            }
                            _ => Cell::from("■").style(Style::default().fg(Color::DarkGray)),
                        }
                    })
                    .collect();
                Row::new(cells)
            })
            .collect();

        let column_constraints = vec![Constraint::Length(2); 53];
        Table::new(rows, column_constraints)
            .column_spacing(0)
            .render(chunks[1], buf);
    }
}
