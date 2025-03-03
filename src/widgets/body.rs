use crate::application::{app::App, ui_state::Focus};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};

pub fn draw(f: &mut Frame, area: Rect, app: &mut App) {
    app.request
        .body
        .set_cursor_style(if app.ui.focus == Focus::Body {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        });

    app.request.body.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Body")
            .title_style(Style::default().fg(if app.ui.focus == Focus::Body {
                Color::Green
            } else {
                Color::default()
            }))
            .border_style(Style::default().fg(if app.ui.focus == Focus::Body {
                Color::Green
            } else {
                Color::DarkGray
            })),
    );
    f.render_widget(&app.request.body, area);
}
