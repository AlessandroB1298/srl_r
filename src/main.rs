mod app;
use std::io;
mod lib;
mod screens;
use std::sync::Arc;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();

    let conn = rusqlite::Connection::open("my_sqllite.db")?;
    let db = Arc::new(conn);

    let app_result = app::App::new(db).run(&mut terminal);

    ratatui::restore();

    match app_result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
