mod app;
mod ui;
mod event_handler;

use crate::app::{App, ResponseData};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use reqwest::Method;
use std::io::{self, stdout};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    // logger初期化
    let log_file = std::sync::Arc::new(std::fs::File::create("./log.log").unwrap());
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_file(true) // default=false
        .with_line_number(true) // default=false
        .with_ansi(true)
        .with_writer(log_file)
        .init();
    tracing::debug!("start turl");

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let methods = vec![Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::TRACE, Method::OPTIONS]; 

    // 非同期でレスポンスを受け取るためのチャンネル
    let (tx, mut rx) = mpsc::channel::<ResponseData>(1);

    loop {
        terminal.draw(|f| ui::draw_ui(f, &mut app))?;

        // イベントの処理
        if event_handler::handle_events(&mut app, &methods, &tx).await? {
            break;
        }

        // レスポンスを受け取った場合に表示
        if let Ok(response) = rx.try_recv() {
                app.response = response;
        }
    }

    // 終了時の処理
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
