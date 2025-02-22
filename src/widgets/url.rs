use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};
use crate::app::{App, Focus};

pub fn draw(f: &mut Frame, area: Rect, app: &mut App) {
    app.url.set_cursor_style(if app.focus == Focus::Url {
        Style::default().add_modifier(Modifier::REVERSED)
    } else {
        Style::default()
    });
    app.url.set_placeholder_text("Enter a URL (e.g. https://httpbin.org/get)");

    app.url.set_block(
        Block::default()
        .borders(Borders::ALL)
        .title("URL")
        .title_style(
            Style::default()
            .fg(if app.focus == Focus::Url {
                Color::Green
            } else {
                Color::default()
            }))
        .border_style(
            Style::default()
            .fg(if app.focus == Focus::Url {
                Color::Green
            } else {
                Color::DarkGray
            })
        )
    );
    f.render_widget(&app.url, area);
}
