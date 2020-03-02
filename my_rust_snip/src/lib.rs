//! cargo-snippetを利用したライブラリ管理。  
//! 
//! ## スニペットの生成
//! 基本的には次のコマンドで生成したJsonファイルをVSCodeのスニペット登録すればOK。
//! 
//! ```shell
//! cargo snippet -t vscode > snip.json  
//! ```
//! 
//! ただし、現時点ではドキュメント用コメントも出力されてしまうため、  
//! それを除去するためgrepを用いて以下のようにフィルタをかけるとよい。
//! 
//! ```shell
//! cargo snippet -t vscode | grep -vG "^\s*\"#\[doc = " > snip.json
//! ```
//! 
//! ## スニペットのテスト
//! また、スニペットのテストは`cargo test`でOK。  
//! ただ、入力系のテストはStdinのシミュレートが上手くできそうになかったので別途行った
//! 
//! ## ドキュメントの生成
//! ドキュメント生成は`cargo doc`で実行される。


/// 数学系
pub mod math;

/// 入出力系
pub mod io;

