//! cargo-snippetを利用したライブラリ管理。  
//! 
//! ## スニペットの生成
//! 基本的には次のコマンドで生成したJsonファイルをVSCodeのスニペット登録すればOK。
//! 
//! ```shell
//! cargo snippet -t vscode > snip.json  
//! ```
//! 
//! 基本的に`//`によるコメントは削除されて出力される。  
//! ただし、現時点ではドキュメント用コメント`///`もDocアトリビュート(`#[doc = \"hogehoge\"]`)で出力されてしまうため、
//! それが煩雑と感じる場合は以下のようにフィルタをかけるとよい。
//! ```shell
//! cargo snippet -t vscode | grep -vG "^\s*\"\s*#\[doc = " > snip.json
//! ```
//! 
//! useするライブラリについては、v0.5.1からprefixで指定できるようになった。
//! 
//! ## スニペットのテスト
//! スニペットのテストは`cargo test`で確認可能。  
//! 
//! ## ドキュメントの生成
//! ドキュメント生成は`cargo doc`で実行される。


/// 数学系
pub mod math;

/// 入出力系
pub mod io;

/// 木
pub mod tree;

/// 集合
pub mod set;