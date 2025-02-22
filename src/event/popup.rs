use crate::app::{App, Focus, PopupFocusState, PopupState};
use crate::utils;
use tui_textarea::{Input, Key};
use std::io;

pub fn handle_events(app: &mut App, input: Input) -> io::Result<bool> {
    match input {
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
        Input { key: Key::Enter, .. } => {
            if app.popup.state == PopupState::Headers {
                app.focus = Focus::Headers;
                app.headers[app.selected_index] = (app.popup.key.lines().concat(), app.popup.value.lines().concat());
                utils::add_empty(&mut app.headers);
            } else if app.popup.state == PopupState::Params {
                app.focus = Focus::Params;
                app.params[app.selected_index] = (app.popup.key.lines().concat(), app.popup.value.lines().concat());
                utils::add_empty(&mut app.params);

            } else {
                app.focus = Focus::None;
            };
            app.popup.state = PopupState::None;
        }
        Input { key: Key::Tab, .. } => {
            app.popup.focus = match app.popup.focus {
                PopupFocusState::Key => PopupFocusState::Value,
                PopupFocusState::Value => PopupFocusState::Key,
            };
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
