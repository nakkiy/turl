use crate::app::{App, Focus};
use crossterm::event::KeyCode;
use std::io;

pub fn handle_events(app: &mut App, code: KeyCode) -> io::Result<bool> {
    match code {
        KeyCode::Char(c) => {
            app.url.push(c);
        }
        KeyCode::Backspace => {
            app.url.pop();
        },
        KeyCode::Esc => {
            if app.focus != Focus::None {
                app.focus = Focus::None;
            }
        },
        _ => {}
    }
    Ok(false)
}
