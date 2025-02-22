use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear},
    Frame,
};
use crate::app::{App, PopupFocusState, PopupState};
use crate::utils;

pub fn draw(f: &mut Frame, app:&mut App) {
    if app.popup.state != PopupState::None {
        let area = utils::centered_rect(50, 25, f.area());
        let popup = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(if app.popup.state == PopupState::Headers{
                "Header"
            } else if app.popup.state == PopupState::Params {
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

        app.popup.key.set_placeholder_text("Enter Key");
        app.popup.key.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title("Key")
            .title_style(
                Style::default()
                .fg(if app.popup.focus == PopupFocusState::Key {
                    Color::Green
                } else {
                    Color::default()
                }))
            .border_style(
                Style::default()
                .fg(if app.popup.focus == PopupFocusState::Key {
                    Color::Green
                } else {
                    Color::DarkGray
                }))
            );
        app.popup.key.set_cursor_style(if app.popup.focus == PopupFocusState::Key {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        });

        app.popup.value.set_placeholder_text("Enter Value");
        app.popup.value.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title("Value")
            .title_style(
                Style::default()
                .fg(if app.popup.focus == PopupFocusState::Value {
                    Color::Green
                } else {
                    Color::default()
                }))
            .border_style(
                Style::default()
                .fg(if app.popup.focus == PopupFocusState::Value {
                    Color::Green
                } else {
                    Color::DarkGray
                }))
        );
        app.popup.value.set_cursor_style(if app.popup.focus == PopupFocusState::Value {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        });
    
        f.render_widget(&app.popup.key, chunks[1]);
        f.render_widget(&app.popup.value, chunks[2]);
    }
}
