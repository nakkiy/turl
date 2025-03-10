use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn clean_up_list(vec: &mut Vec<(String, String)>) {
    vec.retain(|(key, value)| !(key.is_empty() && value.is_empty()));
    if vec
        .last()
        .map_or(true, |(k, v)| !k.is_empty() || !v.is_empty())
    {
        vec.push((String::new(), String::new()));
    }
}
