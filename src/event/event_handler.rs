use crate::app::{App, Focus, PopupState, ResponseData};
use crossterm::event::{self, KeyCode, KeyModifiers};
use reqwest::Method;
use std::io;
use tokio::sync::mpsc;

pub async fn handle_events(
    app: &mut App,
    methods: &[Method],
    tx: &mpsc::Sender<ResponseData>,
) -> io::Result<bool> {
    if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
        if let Ok(event::Event::Key(key_event)) = event::read() {
            match key_event.modifiers {
                KeyModifiers::CONTROL => match key_event.code {
                    KeyCode::Char('j') => {
                        let tx = tx.clone();
                        let mut app_clone = app.clone();
                        tokio::spawn(async move {
                            app_clone.send_request(tx).await;
                        });
                        app.history.add(app.request.clone());
                        return Ok(false);
                    }
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Up => {
                        if let Some(request) = app.history.prev().get_current() {
                            app.request = request.clone();
                            return Ok(false);
                        }
                    }
                    KeyCode::Down => {
                        if let Some(request) = app.history.next().get_current() {
                            app.request = request.clone();
                            return Ok(false);
                        }
                    }
                    _ => {}
                },
                KeyModifiers::ALT => {
                    match key_event.code {
                        KeyCode::Char('m') => {
                            app.selected_index = 0;
                            app.focus = Focus::Method;
                        }
                        KeyCode::Char('u') => {
                            app.selected_index = 0;
                            app.focus = Focus::Url;
                        }
                        KeyCode::Char('h') => {
                            app.selected_index = 0;
                            app.focus = Focus::Headers;
                        }
                        KeyCode::Char('p') => {
                            app.selected_index = 0;
                            app.focus = Focus::Params;
                        }
                        KeyCode::Char('b') => {
                            app.selected_index = 0;
                            app.focus = Focus::Body;
                        }
                        KeyCode::Char('e') => {
                            app.selected_index = 0;
                            app.focus = Focus::ResponseHeaders;
                        }
                        KeyCode::Char('r') => {
                            app.selected_index = 0;
                            app.focus = Focus::ResponseBody;
                        }
                        _ => {}
                    }
                    return Ok(false);
                }
                KeyModifiers::SHIFT => match key_event.code {
                    KeyCode::BackTab => {
                        if app.popup.state == PopupState::None {
                            app.selected_index = 0;
                            app.focus = match app.focus {
                                Focus::Method => Focus::ResponseBody,
                                Focus::ResponseBody => Focus::ResponseHeaders,
                                Focus::ResponseHeaders => Focus::Body,
                                Focus::Body => Focus::Params,
                                Focus::Params => Focus::Headers,
                                Focus::Headers => Focus::Url,
                                Focus::Url => Focus::Method,
                                Focus::None => Focus::ResponseBody,
                                Focus::Popup => Focus::Popup,
                            };
                            return Ok(false);
                        }
                    }
                    _ => {}
                },
                KeyModifiers::NONE => match key_event.code {
                    KeyCode::Tab => {
                        if app.popup.state == PopupState::None {
                            app.selected_index = 0;
                            app.focus = match app.focus {
                                Focus::Method => Focus::Url,
                                Focus::Url => Focus::Headers,
                                Focus::Headers => Focus::Params,
                                Focus::Params => Focus::Body,
                                Focus::Body => Focus::ResponseHeaders,
                                Focus::ResponseHeaders => Focus::ResponseBody,
                                Focus::ResponseBody => Focus::Method,
                                Focus::None => Focus::Method,
                                Focus::Popup => Focus::Popup,
                            };
                            return Ok(false);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            match app.focus {
                Focus::Method => {
                    return crate::event::method::handle_events(app, methods, key_event.code);
                }
                Focus::Url => {
                    return crate::event::url::handle_events(app, key_event.into());
                }
                Focus::Headers => {
                    return crate::event::headers::handle_events(app, key_event);
                }
                Focus::Params => {
                    return crate::event::params::handle_events(app, key_event);
                }
                Focus::Body => {
                    return crate::event::body::handle_events(app, key_event.into());
                }
                Focus::ResponseHeaders => {
                    return crate::event::response_headers::handle_events(app, key_event.code);
                }
                Focus::ResponseBody => {
                    return crate::event::response_body::handle_events(app, key_event.into());
                }
                Focus::Popup => {
                    return crate::event::popup::handle_events(app, key_event);
                }
                _ => {}
            }
        }
    }
    Ok(false)
}
