mod application;
mod event;
mod utils;
mod widgets;

use crate::application::{app::App, response_data::ResponseData};
use crate::widgets::ui;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reqwest::Method;
use std::io::stderr;
use structopt::StructOpt;
use tokio::sync::mpsc;

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

    enable_raw_mode()?;
    let mut stderr = stderr();
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let methods = vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::TRACE,
        Method::OPTIONS,
    ];

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

    Ok(())
}
