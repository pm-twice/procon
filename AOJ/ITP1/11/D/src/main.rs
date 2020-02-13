#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;

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

#[allow(dead_code)]
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

    /// 上面と手前の面から、その時の右の面を返す
    fn question_right(&self, top: u32, front: u32) -> Option<u32> {
        let mut d = Dice::new(&self.surface);
        // とりあえず上面をtopにする
        match d.surface.iter().position(|&i| {i==top}) {
            Some(0) => { ; },
            Some(1) => { d.move_n(); },
            Some(2) => { d.move_w(); },
            Some(3) => { d.move_e(); },
            Some(4) => { d.move_s(); },
            Some(5) => { d.move_s(); d.move_s(); },
            _ => { return None; },
        };

        // frontに応じてrightがどこにあるか探す
        let res = match d.surface.iter().position(|&i| {i==front}) {
            Some(0) => { return None; },
            Some(1) => { d.surface[2] },    // frontが手前→rightは右(2)
            Some(2) => { d.surface[4] },    // frontが右→rightは奥(4)
            Some(3) => { d.surface[1] },    // frontが左→rightは手前(1)
            Some(4) => { d.surface[3] },    // frontが奥→rightは左(3)
            Some(5) => { return None },    // frontが下面→存在しない
            _ => { return None },
        };

        Some(res)
    }

    /// 完全に同じダイスかどうか判定する。
    fn is_same_comp(&self, dice: &Dice) -> bool{
        let mut res = true;
        for (a,b) in self.surface.iter().zip(dice.surface.iter()){
            res &= a==b;
        }
        res
    }

    fn is_same(&self, dice: &Dice) -> bool {
        let mut d = Dice::new(&self.surface);
        let mut res = false;
        // 24パターン総当たりで確認する
        
        for _ in 0..4 {
            for _ in 0..4 {
                res |= d.is_same_comp(&dice);
                d.move_s();
            }
            d.move_e();
        }

        d.move_s();
        d.move_e();

        // 確認の重複あるがひとまずこれで
        for _ in 0..4 {
            for _ in 0..4 {
                res |= d.is_same_comp(&dice);
                d.move_s();
            }
            d.move_e();
        }

        res
    }
}

fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();

    let n = l_iter.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let mut dices: Vec<Dice> = vec![];

    for l in l_iter {
        let v = l.unwrap().split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        dices.push(Dice::new(&v));
    }

    let mut all_diff = true;
    for i in 0..n-1 {
        for j in i+1..n {
            if dices[i].is_same(&dices[j]) == true {
                all_diff = false;
                break;
            }
        }
    }
    match all_diff {
        true => println!("Yes"),
        false => println!("No"),
    };
}
