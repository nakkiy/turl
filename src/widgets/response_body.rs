use crate::application::{app::App, ui_state::Focus};
use ratatui::{
    layout::Rect,
    prelude::{Line, Span},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};

pub fn draw(f: &mut Frame, area: Rect, app: &mut App) {
    let response_title = Line::from(vec![
        Span::raw("Response Body"),
        Span::raw(if !app.response.status.is_empty() {
            " ".to_string()
        } else {
            "".to_string()
        }),
        Span::styled(
            if !app.response.status.is_empty() {
                format!("[{}]", app.response.status)
            } else {
                "".to_string()
            },
            if app.response.status.contains("OK") {
                Style::default().bg(Color::Green).fg(Color::Black)
            } else {
                Style::default().bg(Color::Red).fg(Color::Black)
            },
        ),
    ]);

    app.response
        .body
        .set_cursor_style(if app.ui.focus == Focus::ResponseBody {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        });

    app.response
        .body
        .set_line_number_style(Style::default().fg(Color::DarkGray));

    app.response.body.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title(response_title)
            .title_style(Style::default().fg(if app.ui.focus == Focus::ResponseBody {
                Color::Green
            } else {
                Color::default()
            }))
            .border_style(Style::default().fg(if app.ui.focus == Focus::ResponseBody {
                Color::Green
            } else {
                Color::DarkGray
            })),
    );

    f.render_widget(&app.response.body, area);
}
