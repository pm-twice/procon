use std::io::{self, Read};
use std::str::FromStr;
pub struct Scanner<R: Read> {
    reader: R,
}
impl<R: Read> Scanner<R> {
    pub fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader }
    }
    pub fn read<T: FromStr>(&mut self) -> T {
        let s = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.expect("failed to read char") as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        s.parse::<T>().ok().expect("failed to parse token")
    }
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let q: usize = sc.read();
    let len: usize = q;
    let mut deque: Vec<i64> = vec![0; len];

    // head>tailで空
    // deque[head]がDequeの先頭、deque[tail]がDequeの末尾を指す
    let mut head = len/2;
    let mut tail = head-1;

    // 満杯にならないと仮定して、このチェックは外している
    
    for _ in 0..q {
        let com: u32 = sc.read();
        match com {
            0 => {  // push
                let com2: u32 = sc.read();
                match com2 {
                    0 => {  // to head
                        head = (head - 1 + len) % len;
                        deque[head] = sc.read();
                    },
                    1 => {  // to tail
                        tail = (tail + 1) % len;
                        deque[tail] = sc.read();
                    },
                    _ => {
                        panic!("irregular command2");
                    },
                }
            },
            1 => {  // random
                let p: usize = sc.read();
                let p =  (head + p) % len;
                println!("{}", deque[p]);
            },
            2 => {  // pop
                let com2: u32 = sc.read();
                match com2 {
                    0 => {  // from head
                        head = (head + 1) % len;
                    },
                    1 => {  // from tail
                        tail = (tail - 1 + len) % len;
                    },
                    _ => {
                        panic!("irregular command2");
                    },
                }
            },
            _ => {
                panic!("irregular command");
            },
        }
    }
}
