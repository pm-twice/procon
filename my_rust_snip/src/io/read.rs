use cargo_snippet::snippet;
use std::io::Read;
use std::str::FromStr;

/// Readトレイトから1要素ずつ取り出す関数。lockを用いれば高速読み込みが可能
/// 
/// read_lineは1文字ずつlock&unlockしてしまうので、  
/// lockした上で関数へ渡せば高速で読み取りが可能
/// 
/// @tubo28さんの記事[その1](https://qiita.com/tubo28/items/e6076e9040da57368845#%E5%85%A5%E5%8A%9B%E9%96%A2%E6%95%B0)、[その2](https://qiita.com/tubo28/items/41eeb0dfa1ebbccb1ddc)がベース
/// 
/// # スニペット登録名
/// `snp-read`
/// 
/// # 利用ライブラリ
/// `
/// use std::io::Read;
/// use std::str::FromStr;
/// `
/// 
/// # Example
/// 
/// ```rust
/// use my_rust_snip::io::read;
/// use std::io::Cursor;
/// 
/// // for stdin
/// // let sin = io::stdin();
/// // let mut sin = sin.lock();
///
/// // simulate input with Read Trait
/// let mut sin = Cursor::new("10\n1 2 3 4 5 6 7 8 9 10");
/// 
/// let n: usize = read(&mut sin);
/// let a: Vec<u32> = (0..n).map(|_| read(&mut sin) ).collect();
///
/// assert_eq!(n, 10);
/// assert_eq!(a, vec![1,2,3,4,5,6,7,8,9,10]);
/// ```
#[snippet("snp-read")]
#[snippet(prefix="use std::io::Read;")]
#[snippet(prefix="use std::str::FromStr;")]
pub fn read<T: FromStr, R: Read>(sin: &mut R) -> T {
    // 冒頭の空白を飛ばして、次の空白までのcharを取得し、Stringに構築する
    let s = sin.by_ref().bytes().map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok().expect("failed to parse token")
}

#[cfg(test)]
mod test{
    use super::read;
    use std::io::Cursor;

    #[test]
    fn test_read(){
        // for stdin
        // let sin = io::stdin();
        // let mut sin = sin.lock();
    
        // simulate input with Read Trait
        let mut sin = Cursor::new("10\n1 2 3 4 5 6 7 8 9 10");
    
        let n: usize = read(&mut sin);
        let a: Vec<u32> = (0..n).map(|_| read(&mut sin) ).collect();

        assert_eq!(n, 10);
        assert_eq!(a, vec![1,2,3,4,5,6,7,8,9,10]);
    }
}