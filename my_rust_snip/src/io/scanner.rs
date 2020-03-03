use cargo_snippet::snippet;

use std::io::Read;
use std::str::FromStr;

/// Readトレイトから1要素ずつ取り出すクラス。lockを渡せば高速読み取り可能
/// 
/// スニペットは`scanner`で登録
/// 
/// # Example
/// 
/// ```rust
/// use my_rust_snip::io::Scanner;
/// use std::io::Cursor;
/// 
/// // for stdin
/// // let sin = io::stdin();
/// // let sin = sin.lock();
///
/// // simulate input with Read Trait
/// let sin = Cursor::new("10\n1 2 3 4 5 6 7 8 9 10");
/// 
/// let mut sc = Scanner::new(sin);
/// let n: usize = sc.read();
/// let a: Vec<u32> = (0..n).map(|_| sc.read() ).collect();
///
/// assert_eq!(n, 10);
/// assert_eq!(a, vec![1,2,3,4,5,6,7,8,9,10]);
/// ```
#[snippet("scanner-def")]
pub struct Scanner<R: Read> {
    reader: R,
}

#[snippet("scanner", include="scanner-def")]
impl<R: Read> Scanner<R> {
    pub fn new(reader: R) -> Scanner<R> {
        Scanner {
            reader: reader,
        }
    } 

    pub fn read<T: FromStr>(&mut self) -> T {
        // 冒頭の空白を飛ばして、次の空白までのcharを取得し、Stringに構築する
        let s = self.reader.by_ref().bytes().map(|c| c.expect("failed to read char") as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        s.parse::<T>().ok().expect("failed to parse token")
    }
}

#[cfg(test)]
mod test {
    use super::Scanner;
    use std::io::Cursor;

    #[test]
    fn test_read(){
        // for stdin
        // let sin = io::stdin();
        // let sin = sin.lock();
    
        // simulate input with Read Trait
        let sin = Cursor::new("10\n1 2 3 4 5 6 7 8 9 10");
    
        let mut sc = Scanner::new(sin);
        let n: usize = sc.read();
        let a: Vec<u32> = (0..n).map(|_| sc.read() ).collect();

        assert_eq!(n, 10);
        assert_eq!(a, vec![1,2,3,4,5,6,7,8,9,10]);
    }
}
