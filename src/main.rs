mod app;
use std::io;
mod screens;
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = app::App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
