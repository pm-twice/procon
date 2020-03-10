# 競プロ用Rustライブラリ・スニペット

cargo-snippetを利用したライブラリ管理。  

`cargo snippet -t vscode > snip.json`で生成したJsonファイルをVSCodeのスニペット登録すればOK。  
基本的に`//`によるコメントは削除されて出力される。  
ただし、現時点ではドキュメント用コメント`///`も意図しない形式(`#[doc = \"hogehoge\"]`)で出力されてしまうため、
それを除去するためgrepを用いて以下のようにフィルタをかけるとよい。

```
cargo snippet -t vscode | grep -vG "^\s*\"\s*#\[doc = " > snip.json
```

また、スニペットのテストは`cargo test`でOK。

ドキュメント生成は`cargo doc`で実行される。
