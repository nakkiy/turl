use crate::application::ui_state::{Focus, PopupState};
use crate::application::{app::App, ui_state::PopupFocusState};
use crate::utils;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io;
use tui_textarea::{Input, Key};

pub fn handle_events(app: &mut App, key_event: KeyEvent) -> io::Result<bool> {
    match key_event.modifiers {
        KeyModifiers::SHIFT => match key_event.code {
            KeyCode::BackTab => {
                app.ui.popup.focus = match app.ui.popup.focus {
                    PopupFocusState::Key => PopupFocusState::Value,
                    PopupFocusState::Value => PopupFocusState::Key,
                };
                return Ok(false);
            }
            _ => {}
        },
        KeyModifiers::NONE => match key_event.code {
            KeyCode::Tab => {
                app.ui.popup.focus = match app.ui.popup.focus {
                    PopupFocusState::Key => PopupFocusState::Value,
                    PopupFocusState::Value => PopupFocusState::Key,
                };
                return Ok(false);
            }
            _ => {}
        },
        _ => {}
    }

    match key_event.into() {
        Input { key: Key::Esc, .. } => {
            app.ui.focus = if app.ui.popup.state == PopupState::Headers {
                Focus::Headers
            } else if app.ui.popup.state == PopupState::Params {
                Focus::Params
            } else {
                Focus::None
            };
            app.ui.popup.state = PopupState::None;
        }
        Input {
            key: Key::Enter, ..
        } => {
            if app.ui.popup.state == PopupState::Headers {
                app.ui.focus = Focus::Headers;
                app.request.headers[app.ui.selected_index] = (
                    app.ui.popup.key.lines().concat(),
                    app.ui.popup.value.lines().concat(),
                );
                utils::clean_up_list(&mut app.request.headers);
            } else if app.ui.popup.state == PopupState::Params {
                app.ui.focus = Focus::Params;
                app.request.params[app.ui.selected_index] = (
                    app.ui.popup.key.lines().concat(),
                    app.ui.popup.value.lines().concat(),
                );
                utils::clean_up_list(&mut app.request.params);
            } else {
                app.ui.focus = Focus::None;
            };
            app.ui.popup.state = PopupState::None;
        }
        input => {
            match app.ui.popup.focus {
                PopupFocusState::Key => app.ui.popup.key.input(input),
                PopupFocusState::Value => app.ui.popup.value.input(input),
            };
        }
    }
    Ok(false)
}
