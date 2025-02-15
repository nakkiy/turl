use reqwest::{Client, Method};
use tokio::sync::mpsc;
use ratatui::widgets::ListState;

#[derive(Clone)]
pub struct ResponseData {
    pub status: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

#[derive(Clone)]
pub struct App {
    pub url: String,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub params: Vec<(String, String)>,
    pub body: String,
    pub response: ResponseData,
    pub input_mode: InputMode,
}
#[derive(PartialEq, Clone, Debug)]
pub enum InputMode {
    Url,
    Method,
    Headers,
    Params,
    Body,
    None,
}

impl App {
    pub fn new() -> Self {
        let mut header_list_state = ListState::default();
        header_list_state.select(Some(0));
        Self {
            url: "https://httpbin.org/get".to_string(),
            method: Method::GET,
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            params: vec![("".to_string(), "".to_string())],
            body: "".to_string(),
            response: ResponseData {
                status: "".to_string(),
                headers: vec![],
                body: "".to_string(),
            },
            input_mode: InputMode::None,
        }
    }

    pub async fn send_request(&mut self, tx: mpsc::Sender<ResponseData>) {
        let client = Client::new();
        let mut req = client.request(self.method.clone(), &self.url);

        req = req.header("User-Agent", format!("TURL/{}", env!("CARGO_PKG_VERSION")));
        for (key, value) in &self.headers {
            req = req.header(key, value);
        }

        if self.method == Method::POST || self.method == Method::PUT {
            req = req.body(self.body.clone());
        }

        let response_data = ResponseData {
            status: "".to_string(),
            headers: vec![],
            body: "Sending request...".to_string(),
        };
        let _ = tx.send(response_data).await;

        match req.send().await {
            Ok(res) => {
                tracing::debug!("ctrl + {:?}", res.status());
                let status = res.status().to_string();
                let headers = res.headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect();
                let body = res.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());

                let response_data = ResponseData { status, headers, body };
                let _ = tx.send(response_data).await;
            }
            Err(err) => {
                let response_data = ResponseData {
                    status: "".to_string(),
                    headers: vec![],
                    body: format!("Request failed: {}", err),
                };
                let _ = tx.send(response_data).await;
            }
        }
    }
}
