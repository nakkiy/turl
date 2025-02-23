use crate::app::{App, Focus};
use ratatui::{
    layout::Rect,
    prelude::{Line, Span},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
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
    let block = Paragraph::new(app.response.body.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(response_title)
            .title_style(Style::default().fg(if app.focus == Focus::ResponseBody {
                Color::Green
            } else {
                Color::default()
            }))
            .border_style(Style::default().fg(if app.focus == Focus::ResponseBody {
                Color::Green
            } else {
                Color::DarkGray
            })),
    );
    f.render_widget(block, area);
}
