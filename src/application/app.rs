use reqwest::Client;
use std::fmt::Write;
use tokio::sync::mpsc;
use tui_textarea::TextArea;

use crate::application::{
    history::History, request_data::RequestData, response_data::ResponseData, ui_state::UIState,
};

/// `App` 構造体: アプリケーションの状態を管理する
#[derive(Clone)]
pub struct App {
    pub request: RequestData,   // 現在のリクエストデータ
    pub response: ResponseData, // 最新のレスポンスデータ
    pub ui: UIState,            // UIの状態管理
    pub history: History,       // リクエスト履歴
}

impl App {
    /// `App` の新しいインスタンスを作成
    pub fn new() -> Self {
        Self {
            request: RequestData::new(),
            response: ResponseData::new(),
            ui: UIState::new(),
            history: History::new(100), // 最大100件の履歴を保持
        }
    }

    /// リクエストを送信し、レスポンスを `mpsc::Sender<ResponseData>` 経由で非同期に返す
    pub async fn send_request(&mut self, tx: mpsc::Sender<ResponseData>) {
        let client = Client::new();
        let req = self.request.build_request(&client);

        // 送信中メッセージを送信
        let _ = tx
            .send(ResponseData {
                status: "".to_string(),
                headers: vec![],
                body: TextArea::from_iter(
                    "Sending request...".to_string().lines().map(String::from),
                ),
            })
            .await;

        // リクエスト送信 & レスポンス処理
        match req.send().await {
            Ok(res) => {
                // 成功時のレスポンスを取得
                let response_data = ResponseData::from_success_response(res).await;
                let _ = tx.send(response_data).await;
            }
            Err(err) => {
                // エラー時のレスポンスを作成
                let response_data = ResponseData::from_error(err);
                let _ = tx.send(response_data).await;
            }
        }
    }

    /// レスポンスの HTTP ステータスをフォーマットして標準出力向けの文字列として返す
    pub fn format_stdout_status(&self) -> Result<String, std::fmt::Error> {
        let mut result = String::new();
        writeln!(&mut result, "{}", &self.response.status)?; // ステータスを書き込む
        Ok(result)
    }

    /// レスポンスのヘッダーをフォーマットして標準出力向けの文字列として返す
    pub fn format_stdout_header(&self) -> Result<String, std::fmt::Error> {
        let mut result = String::new();
        for (k, v) in &self.response.headers {
            writeln!(&mut result, "{}: {}", k, v)?; // ヘッダー情報を追加
        }
        Ok(result)
    }

    /// レスポンスのボディをフォーマットして標準出力向けの文字列として返す
    pub fn format_stdout_body(&self) -> Result<String, std::fmt::Error> {
        let mut result = String::new();
        writeln!(&mut result, "{}", &self.response.body.lines().join("\n"))?; // ボディを改行区切りで連結
        Ok(result)
    }
}
