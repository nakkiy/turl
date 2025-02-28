use ratatui::widgets::ListState;
use reqwest::{Client, Method};
use std::fmt::Write;
use tokio::sync::mpsc;
use tui_textarea::TextArea;

use crate::history::History;

#[derive(Clone)]
pub struct ResponseData {
    pub status: String,
    pub headers: Vec<(String, String)>,
    pub body: TextArea<'static>,
}

#[derive(Clone)]
pub struct ListStates {
    pub headers: ListState,
    pub params: ListState,
    pub response_headers: ListState,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Focus {
    Url,
    Method,
    Headers,
    Params,
    Body,
    ResponseHeaders,
    ResponseBody,
    Popup,
    None,
}

#[derive(PartialEq, Clone, Debug)]
pub enum PopupState {
    Headers,
    Params,
    None,
}
#[derive(PartialEq, Clone, Debug)]
pub enum PopupFocusState {
    Key,
    Value,
}

#[derive(Clone)]
pub struct Popup {
    pub state: PopupState,
    pub key: TextArea<'static>,
    pub value: TextArea<'static>,
    pub focus: PopupFocusState,
}

#[derive(Clone)]
pub struct RequestData {
    pub url: TextArea<'static>,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub params: Vec<(String, String)>,
    pub body: TextArea<'static>,
}

#[derive(Clone)]
pub struct App {
    pub request: RequestData,
    pub response: ResponseData,
    pub focus: Focus,
    pub selected_index: usize,
    pub list_states: ListStates,
    pub popup: Popup,
    pub history: History,
}

impl App {
    pub fn new() -> Self {
        let mut header_list_state = ListState::default();
        header_list_state.select(Some(0));
        let mut params_list_state = ListState::default();
        params_list_state.select(Some(0));
        let mut response_header_list_state = ListState::default();
        response_header_list_state.select(Some(0));
        Self {
            request: RequestData {
                url: TextArea::default(),
                method: Method::GET,
                headers: vec![("".to_string(), "".to_string())],
                params: vec![("".to_string(), "".to_string())],
                body: TextArea::default(),
            },
            response: ResponseData {
                status: "".to_string(),
                headers: vec![],
                body: TextArea::default(),
            },
            focus: Focus::None,
            selected_index: 0,
            list_states: ListStates {
                headers: header_list_state,
                params: params_list_state,
                response_headers: response_header_list_state,
            },
            popup: Popup {
                state: PopupState::None,
                key: TextArea::default(),
                value: TextArea::default(),
                focus: PopupFocusState::Key,
            },
            history: History::new(100),
        }
    }

    pub async fn send_request(&mut self, tx: mpsc::Sender<ResponseData>) {
        let client = Client::new();
        let mut req = client.request(
            self.request.method.clone(),
            self.request.url.lines().concat(),
        );

        // クエリパラメータを設定
        let params: Vec<(&str, &str)> = self
            .request
            .params
            .iter()
            .filter(|(k, _)| !k.is_empty()) // 空キーは無視
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        if !params.is_empty() {
            req = req.query(&params);
        }

        // ヘッダーを設定
        req = req.header("User-Agent", format!("TURL/{}", env!("CARGO_PKG_VERSION")));
        for (key, value) in &self.request.headers {
            if key.is_empty() {
                continue;
            }
            req = req.header(key, value);
        }

        // ボディを設定（POST, PUT の場合）
        if self.request.method == Method::POST || self.request.method == Method::PUT {
            req = req.body(self.request.body.lines().concat());
        }

        let response_data = ResponseData {
            status: "".to_string(),
            headers: vec![],
            body: TextArea::from_iter("Sending request...".to_string().lines().map(String::from)),
        };
        let _ = tx.send(response_data).await;

        // リクエスト送信
        match req.send().await {
            Ok(res) => {
                let status = res.status().to_string();
                let headers = res
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect();
                let body = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to read response body".to_string())
                    .replace("\t", "    ");
                let body = TextArea::from_iter(body.lines().map(|s| s.to_string()));

                let response_data = ResponseData {
                    status,
                    headers,
                    body,
                };
                let _ = tx.send(response_data).await;
            }
            Err(err) => {
                let response_data = ResponseData {
                    status: "".to_string(),
                    headers: vec![],
                    body: TextArea::from_iter(
                        format!("Request failed:\n    {}", err)
                            .lines()
                            .map(String::from),
                    ),
                };
                let _ = tx.send(response_data).await;
            }
        }
    }

    pub fn format_stdout_status(&self) -> Result<String, std::fmt::Error> {
        let mut result = String::new();
        writeln!(&mut result, "{}", &self.response.status)?;
        Ok(result)
    }

    pub fn format_stdout_header(&self) -> Result<String, std::fmt::Error> {
        let mut result = String::new();
        for (k, v) in &self.response.headers {
            writeln!(&mut result, "{}: {}", k, v)?;
        }
        Ok(result)
    }

    pub fn format_stdout_body(&self) -> Result<String, std::fmt::Error> {
        let mut result = String::new();
        writeln!(&mut result, "{}", &self.response.body.lines().join("\n"))?;
        Ok(result)
    }
}
