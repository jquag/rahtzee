use std::io;

mod app;
mod event;
mod components;
mod model;
mod theme;
mod score_util;

use app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

