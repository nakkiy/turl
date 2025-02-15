use crate::app::{App, InputMode, ResponseData};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use reqwest::Method;
use tokio::sync::mpsc;
use std::io;

pub async fn handle_events(app: &mut App, methods: &[Method], tx: &mpsc::Sender<ResponseData>) -> io::Result<bool> {
    if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
        if let Ok(event::Event::Key(KeyEvent { code, modifiers, .. })) = event::read() {
            match modifiers {
                KeyModifiers::CONTROL => {
                    tracing::debug!("ctrl + {:?}", code);
                    match code {
                        KeyCode::Char('q') => {
                            return Ok(true);
                        }
                        _ => {}
                    }
                }
                KeyModifiers::ALT => {
                    tracing::debug!("alt + {:?}", code);
                    match code {
                        KeyCode::Char('u') => {
                            app.input_mode = InputMode::Url;
                        }
                        KeyCode::Char('m') => {
                            app.input_mode = InputMode::Method;
                        }
                        KeyCode::Char('p') => {
                            app.input_mode = InputMode::Params;
                        }
                        KeyCode::Char('h') => {
                            app.input_mode = InputMode::Headers;
                        }
                        KeyCode::Char('b') => {
                            app.input_mode = InputMode::Body;
                        }
                        _ => {}
                    }
                }
                KeyModifiers::SHIFT => {
                    tracing::debug!("shift + {:?}", code);
                    match code {
                        KeyCode::BackTab => {
                            app.input_mode = match app.input_mode {
                                InputMode::Method => InputMode::Body,
                                InputMode::Body => InputMode::Params,
                                InputMode::Params => InputMode::Headers,
                                InputMode::Headers => InputMode::Url,
                                InputMode::Url => InputMode::Method,
                                InputMode::None => InputMode::Body,
                            };
                        }
                        _ => {}
                    }
                }
                KeyModifiers::NONE => {
                    tracing::debug!("{:?}", code);
                    tracing::debug!("{:?}", app.input_mode);
                    
                    match code {
                        KeyCode::Esc => {
                            if app.input_mode != InputMode::None {
                                app.input_mode = InputMode::None;
                            }
                        }
                        KeyCode::Tab => {
                            app.input_mode = match app.input_mode {
                                InputMode::Method => InputMode::Url,
                                InputMode::Url => InputMode::Headers,
                                InputMode::Headers => InputMode::Params,
                                InputMode::Params => InputMode::Body,
                                InputMode::Body => InputMode::Method,
                                InputMode::None => InputMode::Method,
                            };
                        }
                        KeyCode::Enter => {
                            let tx = tx.clone();
                            let mut app_clone = app.clone();
                            tokio::spawn(async move {
                                app_clone.send_request(tx).await;
                            });
                        }
                        KeyCode::Char(c) => match app.input_mode {
                            InputMode::Url => {
                                app.url.push(c);
                            }
                            InputMode::Headers => {
                            }
                            InputMode::Params => {
                            }
                            InputMode::Body => {
                                app.body.push(c);
                            }
                            _ => {}
                        },
                        KeyCode::Backspace => match app.input_mode {
                            InputMode::Url => {
                                app.url.pop();
                            }
                            InputMode::Headers => {
                            }
                            InputMode::Params => {
                            }
                            InputMode::Body => {
                                app.body.pop();
                            }
                            _ => {}
                        },
                        KeyCode::Left | KeyCode::Right if app.input_mode == InputMode::Method => {
                            let index = methods.iter().position(|m| m == &app.method).unwrap_or(0);
                            app.method = methods[(index + if code == KeyCode::Right { 1 } else { methods.len() - 1 }) % methods.len()].clone();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // if let Ok(event::Event::Key(KeyEvent { code, .. })) = event::read() {
            // match app.edit_mode {
            //     EditMode::Edit => {
            //         match code {
            //             KeyCode::Esc => {
            //                 app.edit_mode = EditMode::None;
            //             }
            //             KeyCode::Left | KeyCode::Right if app.input_mode == InputMode::Method => {
            //                 let index = methods.iter().position(|m| m == &app.method).unwrap_or(0);
            //                 app.method = methods[(index + if code == KeyCode::Right { 1 } else { methods.len() - 1 }) % methods.len()].clone();
            //             }
            //             KeyCode::Backspace => match app.input_mode {
            //                 InputMode::Url => {
            //                     app.url.pop();
            //                 }
            //                 InputMode::Body => {
            //                     app.body.pop();
            //                 }
            //                 InputMode::Headers => {
            //                     let (key, value) = &mut app.headers[app.editing_header];
            //                     if app.editing_key {
            //                         key.pop();
            //                     } else {
            //                         value.pop();
            //                     }
            //                 }
            //                 _ => {}
            //             },
            //             KeyCode::Char(c) => match app.input_mode {
            //                 InputMode::Url => {
            //                     app.url.push(c);
            //                 }
            //                 InputMode::Body => {
            //                     app.body.push(c);
            //                 }
            //                 InputMode::Headers => {
            //                     let (key, value) = &mut app.headers[app.editing_header];
            //                     if app.editing_key {
            //                         key.push(c);
            //                     } else {
            //                         value.push(c);
            //                     }
            //                 }
            //                 _ => {}
            //             },
            //             KeyCode::Tab if app.input_mode == InputMode::Headers => {
            //                 app.editing_key = !app.editing_key;
            //             }
            //             KeyCode::Up if app.input_mode == InputMode::Headers => {
            //                 if app.editing_header > 0 {
            //                     app.editing_header -= 1;
            //                 }
            //             }
            //             KeyCode::Down if app.input_mode == InputMode::Headers => {
            //                 if app.editing_header < app.headers.len() - 1 {
            //                     app.editing_header += 1;
            //                 }
            //             }
            //             _ => {}
            //         }
            //     }
            //     EditMode::None => {
            //         match code {
            //             KeyCode::Esc => {
            //                 if app.input_mode == InputMode::None {
            //                     return Ok(true);
            //                 } else {
            //                     app.input_mode = InputMode::None;
            //                 }
            //             }
            //             KeyCode::Enter => {
            //                 match app.input_mode {
            //                     InputMode::None => {
            //                         let tx = tx.clone();
            //                         let mut app_clone = app.clone();
            //                         tokio::spawn(async move {
            //                             app_clone.send_request(tx).await;
            //                         });
            //                     }
            //                     _ => {
            //                         app.edit_mode = EditMode::Edit;
            //                     }
            //                 }
            //             }
            //             KeyCode::Tab => {
            //                 app.input_mode = match app.input_mode {
            //                     InputMode::Url => InputMode::Method,
            //                     InputMode::Method => InputMode::Headers,
            //                     InputMode::Headers => InputMode::Body,
            //                     InputMode::Body => InputMode::None,
            //                     InputMode::None => InputMode::Url,
            //                 };
            //             }
            //             _ => {}
            //         }
            //     }
            // }
        // }
    }
    Ok(false)
}
