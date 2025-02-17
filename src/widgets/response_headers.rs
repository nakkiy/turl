use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use crate::app::{App, Focus};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let response_headers_items: Vec<ListItem> = app.response.headers.iter()
    .map(|(k, v)| ListItem::new(format!("{}: {}", k, v)))
    .collect();
    let list = List::new(response_headers_items)
    .block(
        Block::default()
        .title("Response Headers")
        .borders(Borders::ALL)
        .title_style(
            Style::default()
            .fg(if app.focus == Focus::ResponseHeaders {
                Color::Green
            } else {
                Color::default()
            }))
        .border_style(
            Style::default()
            .fg(if app.focus == Focus::ResponseHeaders {
                Color::Green
            } else {
                Color::DarkGray
            })
        )
    );
    f.render_widget(list, area);
}
