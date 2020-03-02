# 競プロ用Rustライブラリ・スニペット

cargo-snippetを利用したライブラリ管理。  

`cargo snippet -t vscode > snip.json`で生成したJsonファイルをVSCodeのスニペット登録すればOK。  
ただし、現時点ではドキュメント用コメントも出力されてしまうため、それを除去するためgrepを用いて以下のようにフィルタをかけるとよい。

```
cargo snippet -t vscode | grep -vG "^\s*\"#\[doc = " > snip.json
```

また、スニペットのテストは`cargo test`でOK。

ドキュメント生成は`cargo doc`で実行される。
