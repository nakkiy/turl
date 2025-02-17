use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
    prelude::{Span, Line},
};
use crate::app::{App, Focus};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let response_title = Line::from(vec![
        Span::raw("Response"),
        Span::raw(if app.response.status != "" {" ".to_string()} else {"".to_string()}),
        Span::styled(if app.response.status != "" {format!("[{}]", app.response.status)} else {"".to_string()},
        if app.response.status.contains("OK") {Style::default().bg(Color::Green)} else {Style::default().bg(Color::Red)}),
        ]);
        let block = Paragraph::new(app.response.body.as_str())
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title(response_title)
            .title_style(
                Style::default()
                .fg(if app.focus == Focus::ResponseBody {
                    Color::Green
                } else {
                    Color::default()
                }))
            .border_style(
                Style::default()
                .fg(if app.focus == Focus::ResponseBody {
                    Color::Green
                } else {
                    Color::DarkGray
                })
            )
            );
    f.render_widget(block, area);
}
