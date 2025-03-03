use crate::application::app::App;
use crate::application::ui_state::Focus;
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
        .params
        .iter()
        .enumerate()
        .map(|(i, (k, v))| {
            let is_selected = app.ui.focus == Focus::Params && i == app.ui.selected_index;
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
            .title("Params")
            .borders(Borders::ALL)
            .title_style(Style::default().fg(if app.ui.focus == Focus::Params {
                Color::Green
            } else {
                Color::default()
            }))
            .border_style(Style::default().fg(if app.ui.focus == Focus::Params {
                Color::Green
            } else {
                Color::DarkGray
            })),
    );

    f.render_stateful_widget(list, area, &mut app.ui.list_states.params);
}
