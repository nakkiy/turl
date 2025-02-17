use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
    prelude::{Span, Line},
};
use crate::app::{App, Focus};

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let block = Paragraph::new(match app.focus {
        Focus::Method =>
            Line::from(vec![
                Span::styled("← /→ ", Style::default().fg(Color::LightBlue)),
                Span::raw(":switch methods "),
                Span::styled("^S", Style::default().fg(Color::LightBlue)),
                Span::raw(":Save "),
                Span::styled("^L", Style::default().fg(Color::LightBlue)),
                Span::raw(":Load "),
                Span::styled("^J", Style::default().fg(Color::LightBlue)),
                Span::raw(":Send "),
                Span::styled("^Q", Style::default().fg(Color::LightBlue)),
                Span::raw(":Quit"),
            ]),
            Focus::Url =>
            Line::from(vec![
                Span::styled("^S", Style::default().fg(Color::LightBlue)),
                Span::raw(":Save "),
                Span::styled("^L", Style::default().fg(Color::LightBlue)),
                Span::raw(":Load "),
                Span::styled("^J", Style::default().fg(Color::LightBlue)),
                Span::raw(":Send "),
                Span::styled("^Q", Style::default().fg(Color::LightBlue)),
                Span::raw(":Quit"),
            ]),
        Focus::Headers =>
            Line::from(vec![
                Span::styled("^S", Style::default().fg(Color::LightBlue)),
                Span::raw(":Save "),
                Span::styled("^L", Style::default().fg(Color::LightBlue)),
                Span::raw(":Load "),
                Span::styled("^J", Style::default().fg(Color::LightBlue)),
                Span::raw(":Send "),
                Span::styled("^Q", Style::default().fg(Color::LightBlue)),
                Span::raw(":Quit"),
            ]),
        Focus::Params =>
            Line::from(vec![
                Span::styled("^S", Style::default().fg(Color::LightBlue)),
                Span::raw(":Save "),
                Span::styled("^L", Style::default().fg(Color::LightBlue)),
                Span::raw(":Load "),
                Span::styled("^J", Style::default().fg(Color::LightBlue)),
                Span::raw(":Send "),
                Span::styled("^Q", Style::default().fg(Color::LightBlue)),
                Span::raw(":Quit"),
            ]),
        Focus::Body =>
            Line::from(vec![
                Span::styled("^S", Style::default().fg(Color::LightBlue)),
                Span::raw(":Save "),
                Span::styled("^L", Style::default().fg(Color::LightBlue)),
                Span::raw(":Load "),
                Span::styled("^J", Style::default().fg(Color::LightBlue)),
                Span::raw(":Send "),
                Span::styled("^Q", Style::default().fg(Color::LightBlue)),
                Span::raw(":Quit"),
            ]),
        Focus::ResponseHeaders =>
            Line::from(vec![
                Span::styled("^S", Style::default().fg(Color::LightBlue)),
                Span::raw(":Save "),
                Span::styled("^L", Style::default().fg(Color::LightBlue)),
                Span::raw(":Load "),
                Span::styled("^J", Style::default().fg(Color::LightBlue)),
                Span::raw(":Send "),
                Span::styled("^Q", Style::default().fg(Color::LightBlue)),
                Span::raw(":Quit"),
            ]),
        Focus::ResponseBody =>
            Line::from(vec![
                Span::styled("^S", Style::default().fg(Color::LightBlue)),
                Span::raw(":Save "),
                Span::styled("^L", Style::default().fg(Color::LightBlue)),
                Span::raw(":Load "),
                Span::styled("^J", Style::default().fg(Color::LightBlue)),
                Span::raw(":Send "),
                Span::styled("^Q", Style::default().fg(Color::LightBlue)),
                Span::raw(":Quit"),
            ]),
        Focus::None =>
            Line::from(vec![
                Span::styled("alt+U", Style::default().fg(Color::LightBlue)),
                Span::raw(":URL "),
                Span::styled("alt+M", Style::default().fg(Color::LightBlue)),
                Span::raw(":Method "),
                Span::styled("alt+H", Style::default().fg(Color::LightBlue)),
                Span::raw(":Headers "),
                Span::styled("alt+B", Style::default().fg(Color::LightBlue)),
                Span::raw(":Body "),
                Span::styled("alt+E", Style::default().fg(Color::LightBlue)),
                Span::raw(":Response Header "),
                Span::styled("alt+R", Style::default().fg(Color::LightBlue)),
                Span::raw(":Response "),
                Span::styled("^S", Style::default().fg(Color::LightBlue)),
                Span::raw(":Save "),
                Span::styled("^L", Style::default().fg(Color::LightBlue)),
                Span::raw(":Load "),
                Span::styled("^J", Style::default().fg(Color::LightBlue)),
                Span::raw(":Send "),
                Span::styled("^Q", Style::default().fg(Color::LightBlue)),
                Span::raw(":Quit"),
            ]),
    });

    f.render_widget(block, area);
}
