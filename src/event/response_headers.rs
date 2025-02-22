use crate::app::{App, Focus};
use crossterm::event::KeyCode;
use std::io;

pub fn handle_events(app: &mut App, code: KeyCode) -> io::Result<bool> {
    match code {
        KeyCode::Down => {
            if app.selected_index < if app.response.headers.len() == 0 { 0 } else { app.response.headers.len() - 1 } {
                app.selected_index = app.selected_index + 1;
            } else {
                app.selected_index = 0;
            }
            app.list_states.response_headers.select(Some(app.selected_index));
        }
        KeyCode::Up => {
            if 0 < app.selected_index {
                app.selected_index = app.selected_index - 1;
            } else {
                app.selected_index = if app.response.headers.len() == 0 { 0 } else { app.response.headers.len() - 1 };
            }
            app.list_states.response_headers.select(Some(app.selected_index));
        }
        KeyCode::Esc => {
            if app.focus != Focus::None {
                app.focus = Focus::None;
            }
        }
        _ => {}
    }
    Ok(false)
}
