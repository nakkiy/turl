use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::{App, Focus};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let block = Paragraph::new(app.body.as_str())
    .block(
        Block::default()
        .borders(Borders::ALL)
        .title("Body")
        .title_style(
            Style::default()
            .fg(if app.focus == Focus::Body {
                Color::Green
            } else {
                Color::default()
            }))
        .border_style(
            Style::default()
            .fg(if app.focus == Focus::Body {
                Color::Green
            } else {
                Color::DarkGray
            })
        )
    );
    f.render_widget(block, area);
}
