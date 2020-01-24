# プロコン用ソースコード

コンテスト参加やオンラインジャッジの学習に用いたソースコードを記録しておく。

## 利用する言語
2019/9の参加まではPythonを利用。  
そのあと少しC++を利用。  
それ以後、学習不足を痛感して螺旋本の学習を開始。(2020･01)  
同時に前から学習したかったRustを利用。

RustはAOJやAtCoderで利用されているバージョンと最新版にずれがあるので、以下のコマンドでディレクトリごとにバージョンを設定するとよい。

```shell
$ mkdir AtCoder
$ cd AtCoder
$ rustup override set 1.15.1
$ rustc --version
rustc 1.15.1 (021bd294c 2017-02-08)
$ cd ../
$ mkdir AOJ
$ cd AOJ
$ rustup override set 1.17.0
$ rustc --version
rustc 1.17.0 (56124baa9 2017-04-24)
$ cd ../
$ rustc --version
rustc 1.39.0 (4560ea788 2019-11-04)
$ rustup toolchain list
stable-x86_64-unknown-linux-gnu (default)
1.15.1-x86_64-unknown-linux-gnu
1.17.0-x86_64-unknown-linux-gnu
```
