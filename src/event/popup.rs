use crate::app::{App, Focus, PopupFocusState, PopupState};
use crate::utils;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io;
use tui_textarea::{Input, Key};

pub fn handle_events(app: &mut App, key_event: KeyEvent) -> io::Result<bool> {
    match key_event.modifiers {
        KeyModifiers::SHIFT => match key_event.code {
            KeyCode::BackTab => {
                app.popup.focus = match app.popup.focus {
                    PopupFocusState::Key => PopupFocusState::Value,
                    PopupFocusState::Value => PopupFocusState::Key,
                };
                return Ok(false);
            }
            _ => {}
        },
        KeyModifiers::NONE => match key_event.code {
            KeyCode::Tab => {
                app.popup.focus = match app.popup.focus {
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
            app.focus = if app.popup.state == PopupState::Headers {
                Focus::Headers
            } else if app.popup.state == PopupState::Params {
                Focus::Params
            } else {
                Focus::None
            };
            app.popup.state = PopupState::None;
        }
        Input {
            key: Key::Enter, ..
        } => {
            if app.popup.state == PopupState::Headers {
                app.focus = Focus::Headers;
                app.headers[app.selected_index] = (
                    app.popup.key.lines().concat(),
                    app.popup.value.lines().concat(),
                );
                utils::clean_up_list(&mut app.headers);
            } else if app.popup.state == PopupState::Params {
                app.focus = Focus::Params;
                app.params[app.selected_index] = (
                    app.popup.key.lines().concat(),
                    app.popup.value.lines().concat(),
                );
                utils::clean_up_list(&mut app.params);
            } else {
                app.focus = Focus::None;
            };
            app.popup.state = PopupState::None;
        }
        input => {
            match app.popup.focus {
                PopupFocusState::Key => app.popup.key.input(input),
                PopupFocusState::Value => app.popup.value.input(input),
            };
        }
    }
    Ok(false)
}
