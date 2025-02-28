use crate::app::RequestData;

#[derive(Clone)]
pub struct History {
    pub entries: Vec<RequestData>,
    pub current_index: Option<usize>,
    pub max_entries: usize, // 履歴の最大数
}

impl History {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            current_index: None,
            max_entries, // 上限を設定
        }
    }

    // 履歴を追加する（上限を超えたら古い履歴を削除）
    pub fn add(&mut self, entry: RequestData) -> Self {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0); // 最も古い履歴を削除
            self.current_index = self.current_index.map(|i| i.saturating_sub(1));
            // インデックスを調整
        }
        self.entries.push(entry);
        self.current_index = Some(self.entries.len() - 1);
        self.clone()
    }

    // 前の履歴を選択
    pub fn prev(&mut self) -> Self {
        if let Some(index) = self.current_index {
            if index > 0 {
                self.current_index = Some(index - 1);
            }
        }
        self.clone()
    }

    // 次の履歴を選択
    pub fn next(&mut self) -> Self {
        if let Some(index) = self.current_index {
            if index + 1 < self.entries.len() {
                self.current_index = Some(index + 1);
            }
        }
        self.clone()
    }

    // 現在の履歴を取得
    pub fn get_current(&self) -> Option<&RequestData> {
        self.current_index.and_then(|idx| self.entries.get(idx))
    }
}
