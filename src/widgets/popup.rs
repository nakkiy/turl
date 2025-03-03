use crate::application::ui_state::PopupState;
use crate::application::{app::App, ui_state::PopupFocusState};
use crate::utils;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    if app.ui.popup.state != PopupState::None {
        let area = utils::centered_rect(50, 25, f.area());
        let popup = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(if app.ui.popup.state == PopupState::Headers {
                "Header"
            } else if app.ui.popup.state == PopupState::Params {
                "Param"
            } else {
                ""
            })
            .title_style(Style::default().fg(Color::Green))
            .style(Style::default().bg(Color::default()));
        f.render_widget(Clear, area);
        f.render_widget(popup, area);

        let area = utils::centered_rect(98, 90, area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(area);

        app.ui
            .popup
            .key
            .set_placeholder_text(if app.ui.popup.state == PopupState::Headers {
                "Enter a key (e.g. Content-Type)"
            } else if app.ui.popup.state == PopupState::Params {
                "Enter a key"
            } else {
                ""
            });
        app.ui.popup.key.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Key")
                .title_style(
                    Style::default().fg(if app.ui.popup.focus == PopupFocusState::Key {
                        Color::Green
                    } else {
                        Color::default()
                    }),
                )
                .border_style(
                    Style::default().fg(if app.ui.popup.focus == PopupFocusState::Key {
                        Color::Green
                    } else {
                        Color::DarkGray
                    }),
                ),
        );
        app.ui
            .popup
            .key
            .set_cursor_style(if app.ui.popup.focus == PopupFocusState::Key {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            });

        app.ui
            .popup
            .value
            .set_placeholder_text(if app.ui.popup.state == PopupState::Headers {
                "Enter a value (e.g. application/json)"
            } else if app.ui.popup.state == PopupState::Params {
                "Enter a value"
            } else {
                ""
            });
        app.ui.popup.value.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Value")
                .title_style(
                    Style::default().fg(if app.ui.popup.focus == PopupFocusState::Value {
                        Color::Green
                    } else {
                        Color::default()
                    }),
                )
                .border_style(Style::default().fg(
                    if app.ui.popup.focus == PopupFocusState::Value {
                        Color::Green
                    } else {
                        Color::DarkGray
                    },
                )),
        );
        app.ui
            .popup
            .value
            .set_cursor_style(if app.ui.popup.focus == PopupFocusState::Value {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            });

        f.render_widget(&app.ui.popup.key, chunks[1]);
        f.render_widget(&app.ui.popup.value, chunks[2]);
    }
}
