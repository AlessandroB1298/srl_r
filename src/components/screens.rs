use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::Modifier;
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::border;
use ratatui::text::Text;
use ratatui::text::{Line, Span};
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, List, ListState, Paragraph};
use ratatui::widgets::{Borders, block};
use ratatui::widgets::{ListItem, Widget};
use std::io;

#[derive(Debug)]
pub enum Screen {
    HomeScreen(HomeScreen),
    SecondScreen(SecondScreen),
}
pub enum Action {
    // Global Actions (Handled by App)
    Quit,
    ShouldSwitch,

    // Screen-Specific Actions (Handled by the current View)
    ScreenSpecific(ScreenAction), // <<< NEW: Holds context-specific actions

    // Fallback/No-Op
    NoOp,
    Default,
}
impl Default for Action {
    fn default() -> Self {
        Action::NoOp
    }
}
pub enum ScreenAction {
    // Menu Actions (Only relevant for SecondScreen)
    MenuNext,
    MenuPrev,
    MenuSelect,
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

#[derive(Debug)]
pub struct SecondScreen {
    pub menu_state: ListState,                 // <<< NEW FIELD
    pub menu_options: &'static [&'static str], // Menu options array
}

impl View for HomeScreen {
    fn handle_events(&mut self) -> io::Result<Action> {
        let mut some_action = Action::Default;
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let result = self.handle_key_event(key_event);
                match result {
                    Action::Quit | Action::ShouldSwitch => {
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
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Enter => Action::ShouldSwitch,

            _ => Action::NoOp,
        }
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Default for SecondScreen {
    fn default() -> Self {
        let mut menu_state = ListState::default();
        // Set the initial selection to the first item (0)
        menu_state.select(Some(0));
        Self {
            menu_state,
            menu_options: &[
                "1. Add New Problem",
                "2. Update Problem",
                "3. List All Problems",
                "4. See Graph of Problems",
            ],
        }
    }
}

impl View for SecondScreen {
    fn handle_events(&mut self) -> io::Result<Action> {
        let mut some_action = Action::Default;
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

        // This is how you correctly render a List (a StatefulWidget)
        ratatui::widgets::StatefulWidget::render(menu_list, menu_area, buf, &mut temp_state);
    }
}
