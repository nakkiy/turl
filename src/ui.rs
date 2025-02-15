use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
    text::{Span, Line},
};
use crate::app::{App, InputMode};

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

    let method_block = Paragraph::new(app.method.as_str())
        .block(
            Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title("Method")
            .title_style(
                Style::default()
                .fg(if app.input_mode == InputMode::Method {
                    Color::Red
                } else {
                    Color::default()
                })
            )
        );
        frame.render_widget(method_block, upper_chunks[0]);

    let url_block = Paragraph::new(app.url.as_str())
        .block(
            Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title("URL")
            .title_style(
                Style::default()
                .fg(if app.input_mode == InputMode::Url {
                    Color::Red
                } else {
                    Color::default()
                })
            )
        );
        frame.render_widget(url_block, upper_chunks[1]);

    let headers_items: Vec<ListItem> = app.headers.iter()
        .map(|(k, v)| ListItem::new(format!("{}: {}", k, v)))
        .collect();
    let headers_list = List::new(headers_items)
        .block(
            Block::default()
            .title("Headers")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title_style(
                Style::default()
                .fg(if app.input_mode == InputMode::Headers {
                    Color::Red
                } else {
                    Color::default()
                })
            )
        );
    frame.render_widget(headers_list, left_chunks[0]);

    let params_items: Vec<ListItem> = app.params.iter()
        .map(|(k, v)| ListItem::new(format!("{}: {}", k, v)))
        .collect();
    let params_list = List::new(params_items)
        .block(
            Block::default()
            .title("Params")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title_style(
                Style::default()
                .fg(if app.input_mode == InputMode::Params {
                    Color::Red
                } else {
                    Color::default()
                })
            )
        );
    frame.render_widget(params_list, left_chunks[1]);
    
    let body_block = Paragraph::new(app.body.as_str())
        .block(
            Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title("Body")
            .title_style(
                Style::default()
                .fg(if app.input_mode == InputMode::Body {
                    Color::Red
                } else {
                    Color::default()
                })
            )
        );
    frame.render_widget(body_block, left_chunks[2]);

    let response_headers_items: Vec<ListItem> = app.response.headers.iter()
        .map(|(k, v)| ListItem::new(format!("{}: {}", k, v)))
        .collect();
    let headers_list = List::new(response_headers_items)
        .block(
            Block::default()
            .title("Response Headers")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title_style(
                Style::default()
                .fg(Color::default())
            )
        );
    frame.render_widget(headers_list, right_chunks[0]);

    let response_title = Line::from(vec![
        Span::raw("Response"),
        Span::raw(if app.response.status != "" {" ".to_string()} else {"".to_string()}),
        Span::styled(if app.response.status != "" {format!("[{}]", app.response.status)} else {"".to_string()},
        if app.response.status.contains("OK") {Style::default().bg(Color::Green)} else {Style::default().bg(Color::Red)}),
    ]);
    let response_block = Paragraph::new(app.response.body.as_str())
        .block(
            Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(response_title)
            .title_style(
                Style::default()
                .fg(Color::default())
            )
        );
    frame.render_widget(response_block, right_chunks[1]);

    let text = Line::from(vec![
        Span::styled("U", Style::default().fg(Color::LightBlue)),
        Span::raw(":URL "),
        Span::styled("M", Style::default().fg(Color::LightBlue)),
        Span::raw(":Method "),
        Span::styled("H", Style::default().fg(Color::LightBlue)),
        Span::raw(":Headers "),
        Span::styled("B", Style::default().fg(Color::LightBlue)),
        Span::raw(":Body "),
        Span::styled("^S", Style::default().fg(Color::LightBlue)),
        Span::raw(":Save "),
        Span::styled("^L", Style::default().fg(Color::LightBlue)),
        Span::raw(":Load "),
        Span::styled("^Q", Style::default().fg(Color::LightBlue)),
        Span::raw(":Quit"),
    ]);
    let footer_block = Paragraph::new(text);
    frame.render_widget(footer_block, chunks[2]);
}
