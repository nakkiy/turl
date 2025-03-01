use ratatui::widgets::ListState;
use tui_textarea::TextArea;

/// `Focus` 列挙型: 現在フォーカスされている UI 要素を表す
#[derive(PartialEq, Clone, Debug)]
pub enum Focus {
    Url,             // URL入力フィールド
    Method,          // HTTPメソッド選択
    Headers,         // ヘッダー一覧
    Params,          // クエリパラメータ一覧
    Body,            // リクエストボディ
    ResponseHeaders, // レスポンスヘッダー一覧
    ResponseBody,    // レスポンスボディ
    Popup,           // ポップアップ表示中
    None,            // フォーカスなし
}

/// `ListStates` 構造体: 各リスト要素の選択状態を管理
#[derive(Clone)]
pub struct ListStates {
    pub headers: ListState,          // ヘッダーリストの状態
    pub params: ListState,           // クエリパラメータリストの状態
    pub response_headers: ListState, // レスポンスヘッダーリストの状態
}

/// `PopupState` 列挙型: ポップアップの状態を表す
#[derive(PartialEq, Clone, Debug)]
pub enum PopupState {
    Headers, // ヘッダー入力ポップアップ
    Params,  // クエリパラメータ入力ポップアップ
    None,    // ポップアップなし
}

/// `PopupFocusState` 列挙型: ポップアップ内の入力フィールドのフォーカスを表す
#[derive(PartialEq, Clone, Debug)]
pub enum PopupFocusState {
    Key,   // キーフィールドにフォーカス
    Value, // バリューフィールドにフォーカス
}

/// `Popup` 構造体: ポップアップの状態を管理
#[derive(Clone)]
pub struct Popup {
    pub state: PopupState,        // 現在のポップアップの状態
    pub key: TextArea<'static>,   // キーフィールド
    pub value: TextArea<'static>, // バリューフィールド
    pub focus: PopupFocusState,   // 現在のフォーカス位置（キー or バリュー）
}

/// `UIState` 構造体: アプリの UI の状態を管理
#[derive(Clone)]
pub struct UIState {
    pub focus: Focus,            // 現在のフォーカスされている UI 要素
    pub selected_index: usize,   // 選択中のリスト要素のインデックス
    pub list_states: ListStates, // 各リストの状態（ヘッダー・パラメータ・レスポンスヘッダー）
    pub popup: Popup,            // ポップアップの状態管理
}

impl UIState {
    /// `UIState` の新しいインスタンスを作成
    /// - 各リストの選択状態を初期化
    /// - 初期フォーカスは `None`
    /// - ポップアップは `None` 状態
    pub fn new() -> Self {
        let mut header_list_state = ListState::default();
        header_list_state.select(Some(0)); // ヘッダーリストの最初の要素を選択

        let mut params_list_state = ListState::default();
        params_list_state.select(Some(0)); // クエリパラメータリストの最初の要素を選択

        let mut response_header_list_state = ListState::default();
        response_header_list_state.select(Some(0)); // レスポンスヘッダーリストの最初の要素を選択

        Self {
            focus: Focus::None, // 初期状態ではフォーカスなし
            selected_index: 0,
            list_states: ListStates {
                headers: header_list_state,
                params: params_list_state,
                response_headers: response_header_list_state,
            },
            popup: Popup {
                state: PopupState::None,     // 初期状態ではポップアップなし
                key: TextArea::default(),    // 空のキーフィールド
                value: TextArea::default(),  // 空のバリューフィールド
                focus: PopupFocusState::Key, // 初期状態ではキーにフォーカス
            },
        }
    }
}
