// use crate::app::{App, Focus, ResponseData};
// use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
// use reqwest::Method;
// use tokio::sync::mpsc;
// use std::io;

// pub async fn handle_events(app: &mut App, methods: &[Method], tx: &mpsc::Sender<ResponseData>) -> io::Result<bool> {
//     if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
//         if let Ok(event::Event::Key(KeyEvent { code, modifiers, .. })) = event::read() {
//             match modifiers {
//                 KeyModifiers::CONTROL => {
//                     tracing::debug!("ctrl + {:?}", code);
//                     match code {
//                         KeyCode::Char('q') => {
//                             return Ok(true);
//                         }
//                         _ => {}
//                     }
//                 }
//                 KeyModifiers::ALT => {
//                     tracing::debug!("alt + {:?}", code);
//                     match code {
//                         KeyCode::Char('u') => {
//                             app.focus = Focus::Url;
//                         }
//                         KeyCode::Char('m') => {
//                             app.focus = Focus::Method;
//                         }
//                         KeyCode::Char('p') => {
//                             app.focus = Focus::Params;
//                         }
//                         KeyCode::Char('h') => {
//                             app.focus = Focus::Headers;
//                         }
//                         KeyCode::Char('b') => {
//                             app.focus = Focus::Body;
//                         }
//                         _ => {}
//                     }
//                 }
//                 KeyModifiers::SHIFT => {
//                     tracing::debug!("shift + {:?}", code);
//                     match code {
//                         KeyCode::BackTab => {
//                             app.focus = match app.focus {
//                                 Focus::Method => Focus::Body,
//                                 Focus::Body => Focus::Params,
//                                 Focus::Params => Focus::Headers,
//                                 Focus::Headers => Focus::Url,
//                                 Focus::Url => Focus::Method,
//                                 Focus::None => Focus::Body,
//                                 _ => Focus::Method,
//                             };
//                         }
//                         _ => {}
//                     }
//                 }
//                 KeyModifiers::NONE => {
//                     tracing::debug!("{:?}", code);
//                     tracing::debug!("{:?}", app.focus);
                    
//                     match code {
//                         KeyCode::Esc => {
//                             if app.focus != Focus::None {
//                                 app.focus = Focus::None;
//                             }
//                         }
//                         KeyCode::Tab => {
//                             app.focus = match app.focus {
//                                 Focus::Method => Focus::Url,
//                                 Focus::Url => Focus::Headers,
//                                 Focus::Headers => Focus::Params,
//                                 Focus::Params => Focus::Body,
//                                 Focus::Body => Focus::response_headers,
//                                 Focus::response_headers => Focus::response_body,
//                                 Focus::response_body => Focus::Method,
//                                 Focus::None => Focus::Method,
//                             };
//                         }
//                         KeyCode::Enter => {
//                             let tx = tx.clone();
//                             let mut app_clone = app.clone();
//                             tokio::spawn(async move {
//                                 app_clone.send_request(tx).await;
//                             });
//                         }
//                         KeyCode::Down => {
//                             if app.focus != Focus::None {
//                                 app.focus = Focus::None;
//                             }
//                         }
//                         KeyCode::Char(c) => match app.focus {
//                             Focus::Url => {
//                                 app.url.push(c);
//                             }
//                             Focus::Headers => {
//                             }
//                             Focus::Params => {
//                             }
//                             Focus::Body => {
//                                 app.body.push(c);
//                             }
//                             _ => {}
//                         },
//                         KeyCode::Backspace => match app.focus {
//                             Focus::Url => {
//                                 app.url.pop();
//                             }
//                             Focus::Headers => {
//                             }
//                             Focus::Params => {
//                             }
//                             Focus::Body => {
//                                 app.body.pop();
//                             }
//                             _ => {}
//                         },
//                         KeyCode::Left | KeyCode::Right if app.focus == Focus::Method => {
//                             let index = methods.iter().position(|m| m == &app.method).unwrap_or(0);
//                             app.method = methods[(index + if code == KeyCode::Right { 1 } else { methods.len() - 1 }) % methods.len()].clone();
//                         }
//                         _ => {}
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }
//     Ok(false)
// }
