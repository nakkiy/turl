use crate::{
    application::{
        app::App,
        ui_state::{Focus, PopupFocusState, PopupState},
    },
    utils,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io;
use tui_textarea::TextArea;

pub fn handle_events(app: &mut App, key_event: KeyEvent) -> io::Result<bool> {
    match key_event.modifiers {
        KeyModifiers::CONTROL => match key_event.code {
            KeyCode::Char('d') => {
                app.request.headers.remove(app.ui.selected_index);
                utils::clean_up_list(&mut app.request.headers);
            }
            _ => {}
        },
        _ => {}
    }
    match key_event.code {
        KeyCode::Down => {
            if app.ui.selected_index < app.request.headers.len() - 1 {
                app.ui.selected_index += 1;
            } else {
                app.ui.selected_index = 0;
            }
            app.ui
                .list_states
                .headers
                .select(Some(app.ui.selected_index));
        }
        KeyCode::Up => {
            if 0 < app.ui.selected_index {
                app.ui.selected_index -= 1;
            } else {
                app.ui.selected_index = app.request.headers.len() - 1;
            }
            app.ui
                .list_states
                .headers
                .select(Some(app.ui.selected_index));
        }
        KeyCode::Enter => {
            app.ui.focus = Focus::Popup;
            app.ui.popup.state = PopupState::Headers;
            app.ui.popup.key = TextArea::new(vec![app.request.headers[app.ui.selected_index]
                .0
                .to_string()]);
            app.ui.popup.value = TextArea::new(vec![app.request.headers[app.ui.selected_index]
                .1
                .to_string()]);
            app.ui.popup.focus = PopupFocusState::Key;
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
