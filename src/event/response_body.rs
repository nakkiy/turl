use crate::app::{App, Focus};
use std::io;
use tui_textarea::{Input, Key};

pub fn handle_events(app: &mut App, input: Input) -> io::Result<bool> {
    match input {
        Input { key: Key::Esc, .. } => {
            if app.focus != Focus::None {
                app.focus = Focus::None;
            }
        }
        Input {
            key: Key::Up | Key::Down | Key::Right | Key::Left,
            ..
        } => {
            app.response.body.input(input);
        }
        _ => {}
    }
    Ok(false)
}
