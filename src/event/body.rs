use crate::application::{app::App, ui_state::Focus};
use std::io;
use tui_textarea::{Input, Key};

pub fn handle_events(app: &mut App, input: Input) -> io::Result<bool> {
    match input {
        Input { key: Key::Esc, .. } => {
            if app.ui.focus != Focus::None {
                app.ui.focus = Focus::None;
            }
        }
        input => {
            app.request.body.input(input);
        }
    }
    Ok(false)
}
