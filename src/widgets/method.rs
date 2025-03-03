use crate::application::{app::App, ui_state::Focus};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let block = Paragraph::new(app.request.method.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Method")
            .title_style(Style::default().fg(if app.ui.focus == Focus::Method {
                Color::Green
            } else {
                Color::default()
            }))
            .border_style(Style::default().fg(if app.ui.focus == Focus::Method {
                Color::Green
            } else {
                Color::DarkGray
            })),
    );
    f.render_widget(block, area);
}
