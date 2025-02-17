use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use crate::app::App;
use crate::widgets::{method, url, headers, params, body, response_headers, response, footer};

pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(frame.size());

    let upper_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Max(20),
            Constraint::Min(10),
        ])
        .split(chunks[0]);

    let lower_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Min(10),
        ])
        .split(lower_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(10),
        ])
        .split(lower_chunks[1]);

    method::draw(frame, upper_chunks[0], app);
    url::draw(frame, upper_chunks[1], app);
    headers::draw(frame, left_chunks[0], app);
    params::draw(frame, left_chunks[1], app);
    body::draw(frame, left_chunks[2], app);
    response_headers::draw(frame, right_chunks[0], app);
    response::draw(frame, right_chunks[1], app);
    footer::draw(frame, chunks[2], app);
}
