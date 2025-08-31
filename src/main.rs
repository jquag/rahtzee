use std::io;

mod app;
mod event;
mod components;
mod model;
mod theme;

use app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

