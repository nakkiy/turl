use crate::app::{App, Focus};
use crossterm::event::KeyCode;
use reqwest::Method;
use std::io;

pub fn handle_events(app: &mut App, methods: &[Method], code: KeyCode) -> io::Result<bool> {
    match code {
        KeyCode::Left | KeyCode::Right if app.focus == Focus::Method => {
            let index = methods.iter().position(|m| m == app.method).unwrap_or(0);
            app.method = methods[(index
                + if code == KeyCode::Right {
                    1
                } else {
                    methods.len() - 1
                })
                % methods.len()]
            .clone();
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
