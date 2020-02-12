#![allow(non_snake_case)]

use std::io;

/// 0: 上面
/// 1: 手前
/// 2: 右
/// 3: 左
/// 4: 奥
/// 5: 下面
#[derive(Debug)]
struct Dice {
    surface: Vec<u32>,
}

impl Dice {
    fn new(surface: &Vec<u32>) -> Dice {
        assert_eq!(surface.len(), 6);
        Dice {
            surface: surface.clone(),
        }
    }

    fn top(&self) -> u32 {
        self.surface[0]
    }

    fn move_s(&mut self){
        let tmp = self.surface[0];
        self.surface[0] = self.surface[4];   // 奥→上面
        self.surface[4] = self.surface[5];   // 下面→奥
        self.surface[5] = self.surface[1];   // 手前→下面
        self.surface[1] = tmp;   // 上面→手前
    }

    fn move_n(&mut self){
        let tmp = self.surface[0];
        self.surface[0] = self.surface[1];   // 手前→上
        self.surface[1] = self.surface[5];   // 下面→手前
        self.surface[5] = self.surface[4];   // 奥→下面
        self.surface[4] = tmp;   // 上面→奥
    }

    fn move_e(&mut self){
        let tmp = self.surface[0];
        self.surface[0] = self.surface[3];   // 左→上面
        self.surface[3] = self.surface[5];   // 下面→左
        self.surface[5] = self.surface[2];   // 右→下面
        self.surface[2] = tmp;  // 上面→右
    }

    fn move_w(&mut self){
        let tmp = self.surface[0];
        self.surface[0] = self.surface[2];   // 右→上面
        self.surface[2] = self.surface[5];   // 下面→右
        self.surface[5] = self.surface[3];   // 左→下面
        self.surface[3] = tmp;  // 上面→左
    }

    fn move_op(&mut self, op: char){
        match op {
            'N' => { self.move_n(); },
            'E' => { self.move_e(); },
            'W' => { self.move_w(); },
            'S' => { self.move_s(); },
            _ => { ; },
        }
    }

    fn move_ops(&mut self, ops: &String){
        for op in ops.chars() {
            self.move_op(op);
        }
    }
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let v = buf.split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut d = Dice::new(&v);

    let mut ops = String::new();
    sin.read_line(&mut ops).ok();
    d.move_ops(&ops);

    println!("{}", d.top());
}
