use crate::{
    app::{App, Focus, PopupFocusState, PopupState},
    utils,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io;
use tui_textarea::TextArea;

pub fn handle_events(app: &mut App, key_event: KeyEvent) -> io::Result<bool> {
    match key_event.modifiers {
        KeyModifiers::CONTROL => match key_event.code {
            KeyCode::Char('d') => {
                app.request.headers.remove(app.selected_index);
                utils::clean_up_list(&mut app.request.headers);
            }
            _ => {}
        },
        _ => {}
    }
    match key_event.code {
        KeyCode::Down => {
            if app.selected_index < app.request.headers.len() - 1 {
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
                app.selected_index = app.request.headers.len() - 1;
            }
            app.list_states.headers.select(Some(app.selected_index));
        }
        KeyCode::Enter => {
            app.focus = Focus::Popup;
            app.popup.state = PopupState::Headers;
            app.popup.key =
                TextArea::new(vec![app.request.headers[app.selected_index].0.to_string()]);
            app.popup.value =
                TextArea::new(vec![app.request.headers[app.selected_index].1.to_string()]);
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
