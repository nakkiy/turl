# turl

`turl`は、ターミナルユーザーインターフェース（TUI）を使用してHTTPリクエストを送信するためのクライアントです。

## インストール

このプロジェクトをクローンして、依存関係をインストールします。

```sh
git clone https://github.com/nakkiy/turl.git
cd turl
cargo run
```

## 使用方法
`turl`を実行するには、以下のコマンドを使用します。
```sh
turl
```

## オプション
`turl`は以下のコマンドラインオプションをサポートしています。  
終了時に標準出力に出力します。
- `-s`, `--status`: ステータスを表示
- `-h`, `--headers`: ヘッダーを表示
- `-b`, `--body`: ボディを表示

例:
```sh
cargo run -- --status --headers --body
```

## ライセンス
このプロジェクトはMITライセンスの下でライセンスされています。詳細については、[LICENSE](LICENSE)ファイルを参照してください。
