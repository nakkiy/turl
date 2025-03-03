use reqwest::{Client, Method, RequestBuilder};
use tui_textarea::TextArea;

/// `RequestData` 構造体: HTTPリクエストの情報を保持する
#[derive(Clone)]
pub struct RequestData {
    pub url: TextArea<'static>,         // リクエストのURL
    pub method: Method,                 // HTTPメソッド (GET, POST, PUT など)
    pub headers: Vec<(String, String)>, // HTTPヘッダー
    pub params: Vec<(String, String)>,  // クエリパラメータ
    pub body: TextArea<'static>,        // リクエストボディ（POST, PUT の場合のみ）
}

impl RequestData {
    /// `RequestData` の新しいインスタンスを作成
    /// デフォルトは `GET` メソッドで、空のURL・ヘッダー・パラメータ・ボディを持つ
    pub fn new() -> Self {
        Self {
            url: TextArea::default(),
            method: Method::GET,
            headers: vec![("".to_string(), "".to_string())], // 空のヘッダー
            params: vec![("".to_string(), "".to_string())],  // 空のクエリパラメータ
            body: TextArea::default(),
        }
    }

    /// `RequestBuilder` を作成し、リクエストを構築する
    ///
    /// # 引数
    /// - `client` - `reqwest::Client` インスタンス
    ///
    /// # 戻り値
    /// - `RequestBuilder` - 構築済みのリクエスト
    pub fn build_request(&self, client: &Client) -> RequestBuilder {
        let req = self.initialize_request(client);
        let req = self.set_query_params(req);
        let req = self.set_headers(req);
        self.set_body(req)
    }

    /// `reqwest::Client` を使って基本のリクエストを作成する
    fn initialize_request(&self, client: &Client) -> RequestBuilder {
        client.request(self.method.clone(), self.url.lines().concat())
    }

    /// クエリパラメータをリクエストに追加する
    ///
    /// # 引数
    /// - `req` - クエリパラメータを追加する `RequestBuilder`
    ///
    /// # 戻り値
    /// - `RequestBuilder` - クエリパラメータが追加されたリクエスト
    fn set_query_params(&self, req: RequestBuilder) -> RequestBuilder {
        let params: Vec<(&str, &str)> = self
            .params
            .iter()
            .filter(|(k, _)| !k.is_empty()) // 空キーのパラメータは無視
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        if params.is_empty() {
            req
        } else {
            req.query(&params)
        }
    }

    /// HTTPヘッダーをリクエストに追加する
    ///
    /// # 引数
    /// - `req` - ヘッダーを追加する `RequestBuilder`
    ///
    /// # 戻り値
    /// - `RequestBuilder` - ヘッダーが追加されたリクエスト
    fn set_headers(&self, mut req: RequestBuilder) -> RequestBuilder {
        // `User-Agent` をデフォルトで設定
        req = req.header("User-Agent", format!("TURL/{}", env!("CARGO_PKG_VERSION")));

        // ヘッダーを追加（空キーは無視）
        for (key, value) in &self.headers {
            if !key.is_empty() {
                req = req.header(key, value);
            }
        }
        req
    }

    /// ボディをリクエストに追加する（POST, PUT の場合のみ）
    ///
    /// # 引数
    /// - `req` - ボディを追加する `RequestBuilder`
    ///
    /// # 戻り値
    /// - `RequestBuilder` - ボディが追加されたリクエスト（GET, DELETE などは変更なし）
    fn set_body(&self, req: RequestBuilder) -> RequestBuilder {
        if self.method == Method::POST || self.method == Method::PUT {
            req.body(self.body.lines().concat()) // 改行を削除して結合
        } else {
            req
        }
    }
}
