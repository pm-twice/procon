use cargo_snippet::snippet;

use std::io::{Read, StdinLock};
use std::str::FromStr;

/// lockを用いた高速なReader
/// 
/// read_lineは1文字ずつlock&unlockしてしまうので、まとめて読み込む
/// 
/// @tubo28さんの記事[その1](https://qiita.com/tubo28/items/e6076e9040da57368845#%E5%85%A5%E5%8A%9B%E9%96%A2%E6%95%B0)、[その2](https://qiita.com/tubo28/items/41eeb0dfa1ebbccb1ddc)がベース
/// 
/// # 使用例
/// ```rust, ignore
/// use std::io::{self, Read, StdinLock};
/// use std::str::FromStr;
/// use my_rust_snip::io::read;
/// 
/// let sin = io::stdin();
/// let mut slock = sin.lock();
/// let s = &mut slock;
///
/// let n: usize = read(s);
/// let a: Vec<u32> = (0..n).map(|_| read(s)).collect();
///
/// println!("{}", n);
/// println!("{:?}", a);
///
/// ```
#[snippet("myio")]
#[snippet("read")]
pub fn read<T: FromStr>(slock: &mut StdinLock) -> T {
    // 冒頭の空白を飛ばして、次の空白までのcharを取得し、Stringに構築する
    let s = slock.by_ref().bytes().map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok().expect("failed to parse token")
}