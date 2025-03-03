use crate::application::{app::App, ui_state::Focus};
use crossterm::event::KeyCode;
use std::io;

pub fn handle_events(app: &mut App, code: KeyCode) -> io::Result<bool> {
    match code {
        KeyCode::Down => {
            if app.ui.selected_index
                < if app.response.headers.is_empty() {
                    0
                } else {
                    app.response.headers.len() - 1
                }
            {
                app.ui.selected_index += 1;
            } else {
                app.ui.selected_index = 0;
            }
            app.ui
                .list_states
                .response_headers
                .select(Some(app.ui.selected_index));
        }
        KeyCode::Up => {
            if 0 < app.ui.selected_index {
                app.ui.selected_index -= 1;
            } else {
                app.ui.selected_index = if app.response.headers.is_empty() {
                    0
                } else {
                    app.response.headers.len() - 1
                };
            }
            app.ui
                .list_states
                .response_headers
                .select(Some(app.ui.selected_index));
        }
        KeyCode::Esc => {
            if app.ui.focus != Focus::None {
                app.ui.focus = Focus::None;
            }
        }
        _ => {}
    }
    Ok(false)
}
