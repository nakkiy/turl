use crate::app::{App, Focus};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let block = Paragraph::new(app.method.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Method")
            .title_style(Style::default().fg(if app.focus == Focus::Method {
                Color::Green
            } else {
                Color::default()
            }))
            .border_style(Style::default().fg(if app.focus == Focus::Method {
                Color::Green
            } else {
                Color::DarkGray
            })),
    );
    f.render_widget(block, area);
}
