use crate::app::{App, Focus};
use ratatui::text::{Line, Span};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw(f: &mut Frame, area: Rect, app: &mut App) {
    let headers_items: Vec<ListItem> = app
        .request
        .headers
        .iter()
        .enumerate()
        .map(|(i, (k, v))| {
            let is_selected = app.focus == Focus::Headers && i == app.selected_index;
            let style = if is_selected {
                Style::default().fg(Color::Black).bg(Color::LightBlue) // 選択時の色
            } else {
                Style::default()
            };

            let key_style = if is_selected {
                Style::default().fg(Color::White).bg(Color::LightBlue) // 選択時は白
            } else {
                Style::default().fg(Color::Green) // 通常時は青
            };

            let line = Line::from(vec![
                Span::styled(format!("{}: ", k), key_style), // キーの色を変更
                Span::styled(v.to_string(), style),          // 値はデフォルトのスタイル
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(headers_items).block(
        Block::default()
            .title("Headers")
            .borders(Borders::ALL)
            .title_style(Style::default().fg(if app.focus == Focus::Headers {
                Color::Green
            } else {
                Color::default()
            }))
            .border_style(Style::default().fg(if app.focus == Focus::Headers {
                Color::Green
            } else {
                Color::DarkGray
            })),
    );

    f.render_stateful_widget(list, area, &mut app.list_states.headers);
}
