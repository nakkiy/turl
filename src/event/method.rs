use crate::application::{app::App, ui_state::Focus};
use crossterm::event::KeyCode;
use reqwest::Method;
use std::io;

pub fn handle_events(app: &mut App, methods: &[Method], code: KeyCode) -> io::Result<bool> {
    match code {
        KeyCode::Left | KeyCode::Right if app.ui.focus == Focus::Method => {
            let index = methods
                .iter()
                .position(|m| m == app.request.method)
                .unwrap_or(0);
            app.request.method = methods[(index
                + if code == KeyCode::Right {
                    1
                } else {
                    methods.len() - 1
                })
                % methods.len()]
            .clone();
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
