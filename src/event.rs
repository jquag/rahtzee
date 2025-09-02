use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::app::App;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    // Only use short timeout when rolling, otherwise block waiting for events
    let timeout = if app.is_rolling() {
        app.update_dice_animation();
        Duration::from_millis(100)
    } else {
        Duration::from_secs(30) // Long timeout when not animating
    };
    
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                handle_key_event(app, key_event)
            }
            _ => {}
        }
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('r') => {
            if !app.is_rolling() {
                app.start_roll();
            }
        },
        KeyCode::Char('1'..='5') => {
            if !app.is_rolling() {
                if let KeyCode::Char(c) = key_event.code {
                    if let Some(digit) = c.to_digit(10) {
                        app.toggle_hold((digit - 1) as usize);
                    }
                }
            }
        },
        KeyCode::Char('l') | KeyCode::Right =>
            app.rolls.select_next(),
        KeyCode::Char('h') | KeyCode::Left =>
            app.rolls.select_prev(),
        _ => {}
    }
}
