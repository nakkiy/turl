use super::request_data::RequestData;

/// `History` 構造体: リクエスト履歴を管理
#[derive(Clone)]
pub struct History {
    pub entries: Vec<RequestData>,    // 保存されているリクエスト履歴
    pub current_index: Option<usize>, // 現在選択されている履歴のインデックス（None は履歴なし）
    pub max_entries: usize,           // 履歴の最大保存数
}

impl History {
    /// 新しい `History` インスタンスを作成
    ///
    /// # 引数
    /// - `max_entries` - 保存する履歴の最大数
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            current_index: None,
            max_entries, // 上限を設定
        }
    }

    /// 新しいリクエストを履歴に追加
    ///
    /// - 上限を超えた場合、最も古い履歴を削除する
    /// - 追加後、新しい履歴を選択状態にする
    ///
    /// # 引数
    /// - `entry` - 追加する `RequestData`
    ///
    /// # 戻り値
    /// - 更新された `History` インスタンス
    pub fn add(&mut self, entry: RequestData) -> Self {
        // 履歴の最大数を超えた場合、最も古いエントリを削除
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0); // 先頭（最も古い履歴）を削除
            self.current_index = self.current_index.map(|i| i.saturating_sub(1));
            // インデックスを調整（0より小さくならないように `saturating_sub` を使用）
        }

        // 新しい履歴を追加し、選択中のインデックスを更新
        self.entries.push(entry);
        self.current_index = Some(self.entries.len() - 1);

        self.clone()
    }

    /// ひとつ前の履歴を選択
    ///
    /// - `current_index` を 1 つ前に移動する（既に先頭の場合は変更なし）
    ///
    /// # 戻り値
    /// - 更新された `History` インスタンス
    pub fn prev(&mut self) -> Self {
        if let Some(index) = self.current_index {
            if index > 0 {
                self.current_index = Some(index - 1);
            }
        }
        self.clone()
    }

    /// ひとつ次の履歴を選択
    ///
    /// - `current_index` を 1 つ後ろに移動する（既に最新の履歴の場合は変更なし）
    ///
    /// # 戻り値
    /// - 更新された `History` インスタンス
    pub fn next(&mut self) -> Self {
        if let Some(index) = self.current_index {
            if index + 1 < self.entries.len() {
                self.current_index = Some(index + 1);
            }
        }
        self.clone()
    }

    /// 現在の履歴を取得
    ///
    /// # 戻り値
    /// - `Some(&RequestData)` : 選択中のリクエスト履歴
    /// - `None` : 履歴がない場合
    pub fn get_current(&self) -> Option<&RequestData> {
        self.current_index.and_then(|idx| self.entries.get(idx))
    }
}
