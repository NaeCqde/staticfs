# staticfs の仕様

`staticfs HOST PORT`: カレントディレクトリのファイルを HTTP で配る。設定ファイルも環境変数もない。

## コマンド

- `staticfs HOST PORT`: 待ち受けて配る。停止は Ctrl-C か SIGTERM(実行中の通信を終えてから止まる)。
- `staticfs -h` / `--help`: 使い方を出す(終了コード 0)。
- 引数が足りない・PORT が数字でない・範囲外: 理由と使い方を出して終了コード 2。起動に失敗(ポートが使用中など)したら終了コード 1。

ログは `RUST_LOG` で調整できる(既定は `info`)。

## 配るもの

- ディレクトリは一覧を出す。ファイルは中身を返す(Range、ETag、Last-Modified に対応。`ntex-files` の機能)。
- `.` で始まる名前(`.env`、`.git` など)は、配らず、一覧にも出さない。
- 起動した場所の外を指すシンボリックリンクは、配らず、一覧にも出さない(`ntex-files` はリンクの行き先を確かめないので、`confine.rs` で止めている)。外へ出るリクエストは 404 / 405 になる。
- 一覧のページは自前(`listing.rs`)。リンクは `%XX` で正しく符号化し、名前は HTML としてエスケープする。ファイル名に `"` や `#` があっても、リンクは壊れず、属性も足せない。`Content-Security-Policy` でスクリプトを許さない。
- 全レスポンスに `X-Content-Type-Options: nosniff`。

## 構成(`src/`)

`main`(起動)、`cli`(引数)、`confine`(配る範囲の制限)、`listing`(一覧ページ)。`confine.rs` と `listing.rs` の一部は `seamless-fs` と同じ。

## ビルド

`cargo build --release`、`cargo test`。

## セキュリティ上の注意

- 認証も暗号化もない。公開するときは、前にリバースプロキシを置く。
- 配るのはカレントディレクトリ全体。秘密のファイルは置かないか、`.` で始まる名前にする。
