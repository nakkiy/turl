use crate::app::{App, Focus, ResponseData};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use reqwest::Method;
use tokio::sync::mpsc;
use std::io;

pub async fn handle_events(app: &mut App, methods: &[Method], tx: &mpsc::Sender<ResponseData>) -> io::Result<bool> {
    if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
        if let Ok(event::Event::Key(KeyEvent { code, modifiers, .. })) = event::read() {

            tracing::debug!("mod*{:?}, key*{:?}", modifiers, code);

            match modifiers {
                KeyModifiers::CONTROL => {
                    match code {
                        KeyCode::Char('j') => {
                            let tx = tx.clone();
                            let mut app_clone = app.clone();
                            tokio::spawn(async move {
                                app_clone.send_request(tx).await;
                            });
                            return Ok(false);
                        }
                        KeyCode::Char('q') => {
                            return Ok(true);
                        }
                        _ => {}
                    }
                }
                KeyModifiers::SHIFT => {
                    match code {
                        KeyCode::BackTab => {
                            app.focus = match app.focus {
                                Focus::Method => Focus::ResponseBody,
                                Focus::ResponseBody => Focus::ResponseHeaders,
                                Focus::ResponseHeaders => Focus::Body,
                                Focus::Body => Focus::Params,
                                Focus::Params => Focus::Headers,
                                Focus::Headers => Focus::Url,
                                Focus::Url => Focus::Method,
                                Focus::None => Focus::ResponseBody,
                            };
                            return Ok(false);
                        }
                        _ => {}
                    }
                }
                KeyModifiers::ALT => {
                    match code {
                        KeyCode::Char('m') => app.focus = Focus::Method,
                        KeyCode::Char('u') => app.focus = Focus::Url,
                        KeyCode::Char('h') => app.focus = Focus::Headers,
                        KeyCode::Char('p') => app.focus = Focus::Params,
                        KeyCode::Char('b') => app.focus = Focus::Body,
                        KeyCode::Char('e') => app.focus = Focus::ResponseHeaders,
                        KeyCode::Char('r') => app.focus = Focus::ResponseBody,
                        _ => {}
                    }
                    return Ok(false);
                }
                KeyModifiers::NONE => {
                    match code {
                        KeyCode::Tab => {
                            app.focus = match app.focus {
                                Focus::Method => Focus::Url,
                                Focus::Url => Focus::Headers,
                                Focus::Headers => Focus::Params,
                                Focus::Params => Focus::Body,
                                Focus::Body => Focus::ResponseHeaders,
                                Focus::ResponseHeaders => Focus::ResponseBody,
                                Focus::ResponseBody => Focus::Method,
                                Focus::None => Focus::Method,
                            };
                            return Ok(false);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            match app.focus {
                Focus::Method => {
                    return crate::event::method::handle_events(app, methods, code);
                }
                Focus::Url => {
                    return crate::event::url::handle_events(app, code);
                }
                Focus::Headers => {
                    return crate::event::headers::handle_events(app, code);
                }
                Focus::Params => {
                    return crate::event::params::handle_events(app, code);
                }
                _ => {}
            }
        }
    }
    Ok(false)
}
