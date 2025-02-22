mod app;
mod ui;
mod event;
mod widgets;
mod utils;

use crate::app::{App, ResponseData};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use reqwest::Method;
use std::io::stderr;
use tokio::sync::mpsc;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "turl", about = "TUI HTTP client.")]
struct Opt {
    /// Show status
    #[structopt(short = "s", long = "status")]
    show_status: bool,

    /// Show headers
    #[structopt(short = "h", long = "headers")]
    show_headers: bool,

    /// Show body
    #[structopt(short = "b", long = "body")]
    show_body: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

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
    let mut stderr = stderr();
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let methods = vec![Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::TRACE, Method::OPTIONS]; 

    // 非同期でレスポンスを受け取るためのチャンネル
    let (tx, mut rx) = mpsc::channel::<ResponseData>(1);

    loop {
        terminal.draw(|f| ui::draw_ui(f, &mut app))?;

        // イベントの処理
        if event::event_handler::handle_events(&mut app, &methods, &tx).await? {
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

    if opt.show_status {
        println!("{}", app.format_stdout_status()?);
    }
    if opt.show_headers {
        println!("{}", app.format_stdout_header()?);
    }
    if opt.show_body {
        println!("{}", app.format_stdout_body()?);
    }

    tracing::debug!("start end");
    Ok(())
}
