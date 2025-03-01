use reqwest::Response;
use tui_textarea::TextArea;

/// `ResponseData` 構造体: HTTPレスポンスのデータを保持する
#[derive(Clone)]
pub struct ResponseData {
    pub status: String,                 // HTTP ステータスコード（例: "200 OK"）
    pub headers: Vec<(String, String)>, // HTTP ヘッダーのリスト
    pub body: TextArea<'static>,        // レスポンスボディ（テキスト形式）
}

impl ResponseData {
    /// `ResponseData` の新しいインスタンスを作成（空のデフォルト値）
    pub fn new() -> Self {
        Self {
            status: "".to_string(),
            headers: vec![],
            body: TextArea::default(),
        }
    }

    /// HTTPレスポンス (`reqwest::Response`) から `ResponseData` を作成
    ///
    /// # 引数
    /// - `res` - `reqwest::Response` のインスタンス
    ///
    /// # 戻り値
    /// - `ResponseData` インスタンス
    pub async fn from_success_response(res: Response) -> Self {
        let status = res.status().to_string(); // HTTP ステータスを取得
        let headers = Self::parse_headers(&res); // ヘッダーを解析
        let body = Self::parse_body(res).await; // ボディを取得

        Self {
            status,
            headers,
            body,
        }
    }

    /// エラー (`reqwest::Error`) から `ResponseData` を作成
    ///
    /// - ステータスコードやヘッダーは空になる
    /// - エラーメッセージをボディに格納
    ///
    /// # 引数
    /// - `err` - `reqwest::Error`
    ///
    /// # 戻り値
    /// - `ResponseData` インスタンス（エラー情報を含む）
    pub fn from_error(err: reqwest::Error) -> Self {
        let body = Self::format_error_message(&err); // エラーメッセージを整形

        Self {
            status: "".to_string(),
            headers: vec![],
            body,
        }
    }

    /// HTTPレスポンスのヘッダーを解析し、キーと値のペアのリストを作成する
    ///
    /// # 引数
    /// - `res` - `reqwest::Response` の参照
    ///
    /// # 戻り値
    /// - `(String, String)` のリスト（ヘッダーのキーと値）
    fn parse_headers(res: &Response) -> Vec<(String, String)> {
        res.headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect()
    }

    /// HTTPレスポンスのボディを非同期で取得し、テキストエリアに変換
    ///
    /// # 引数
    /// - `res` - `reqwest::Response`
    ///
    /// # 戻り値
    /// - `TextArea<'static>`（ボディの内容を含む）
    async fn parse_body(res: Response) -> TextArea<'static> {
        let body_text = res
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read response body".to_string()) // 取得に失敗した場合のデフォルト値
            .replace("\t", "    "); // タブをスペースに変換（整形しやすくするため）

        TextArea::from_iter(body_text.lines().map(|s| s.to_string())) // 各行を `TextArea` にセット
    }

    /// エラーメッセージをフォーマットし、`TextArea` に格納
    ///
    /// # 引数
    /// - `err` - `reqwest::Error`
    ///
    /// # 戻り値
    /// - `TextArea<'static>`（エラー情報を含む）
    fn format_error_message(err: &reqwest::Error) -> TextArea<'static> {
        TextArea::from_iter(
            format!("Request failed:\n    {}", err)
                .lines()
                .map(String::from),
        )
    }
}
