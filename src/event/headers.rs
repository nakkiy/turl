use crate::app::{App, Focus, PopupFocusState, PopupState};
use crossterm::event::KeyCode;
use std::io;
use tui_textarea::TextArea;

pub fn handle_events(app: &mut App, code: KeyCode) -> io::Result<bool> {
    match code {
        KeyCode::Down => {
            if app.selected_index < app.headers.len() - 1 {
                app.selected_index += 1;
            } else {
                app.selected_index = 0;
            }
            app.list_states.headers.select(Some(app.selected_index));
        }
        KeyCode::Up => {
            if 0 < app.selected_index {
                app.selected_index -= 1;
            } else {
                app.selected_index = app.headers.len() - 1;
            }
            app.list_states.headers.select(Some(app.selected_index));
        }
        KeyCode::Enter => {
            app.focus = Focus::Popup;
            app.popup.state = PopupState::Headers;
            app.popup.key = TextArea::new(vec![app.headers[app.selected_index].0.to_string()]);
            app.popup.value = TextArea::new(vec![app.headers[app.selected_index].1.to_string()]);
            app.popup.focus = PopupFocusState::Key;
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
