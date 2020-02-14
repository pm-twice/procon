#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;

/// u:上, f:前,l:左, b:奥, r:右, d:下
#[derive(Debug)]
struct Dice {
    u: u32, f: u32, r: u32, l: u32, b: u32, d: u32, 
}

#[allow(dead_code)]
impl Dice {
    fn new(u: u32, f: u32, r: u32, l: u32, b: u32, d: u32) -> Dice {
        Dice {u, f, r, l, b, d}
    }

    fn from_vec(vec: &Vec<u32>) -> Dice {
        assert_eq!(vec.len(), 6);
        Dice {
            u: vec[0], f: vec[1], r: vec[2], l: vec[3], b: vec[4], d: vec[5],
        }
    }

    fn from_dice(dice: &Dice) -> Dice {
        Dice {u: dice.u, f: dice.f, r: dice.r, l: dice.l, b: dice.b, d: dice.d, }
    }

    fn move_s(&mut self){
        let tmp = self.u;
        self.u = self.b;
        self.b = self.d;
        self.d = self.f;
        self.f = tmp;
    }

    fn move_n(&mut self) {
        let tmp = self.u;
        self.u = self.f;
        self.f = self.d;
        self.d = self.b;
        self.b = tmp;
    }

    fn move_e(&mut self) {
        let tmp = self.u;
        self.u = self.l;
        self.l = self.d;
        self.d = self.r;
        self.r = tmp;
    }

    fn move_w(&mut self) {
        let tmp = self.u;
        self.u = self.r;
        self.r = self.d;
        self.d = self.l;
        self.l = tmp;
    }

    fn rotate_r(&mut self){
        let tmp = self.f;
        self.f = self.l;
        self.l = self.b;
        self.b = self.r;
        self.r = tmp;
    }

    fn rotate_l(&mut self){
        let tmp = self.f;
        self.f = self.r;
        self.r = self.b;
        self.b = self.l;
        self.l = tmp;
    }

    fn operate(&mut self, op: char){
        match op {
            'S' => self.move_s(),
            'N' => self.move_n(),
            'W' => self.move_w(),
            'E' => self.move_e(),
            'R' => self.rotate_r(),
            'L' => self.rotate_l(),
            _ => (),
        };
    }

    fn operate_string(&mut self, ops: &String) {
        for c in ops.chars() {
            self.operate(c);
        }
    }

    fn top(&self) -> u32 {
        self.u
    }

    fn question_right(&self, top: u32, front: u32) -> Option<u32> {
        let mut v = (0..6).map(|_| Dice::from_dice(&self)).collect::<Vec<Dice>>();

        // 異なる上面6パターンを得る
        v[1].move_e();
        v[2].move_w();
        v[3].move_n();
        v[4].move_s();
        v[5].move_s(); v[5].move_s();

        for d in v.iter_mut() {
            // 横回転4パターンで全てを探索
            for _ in 0..4 {
                if d.u == top && d.f == front {
                    return Some(d.r);
                }
                d.rotate_r();
            }
        }

        None
    }

    fn is_same(&self, dice: &Dice) -> bool {
        let mut v = (0..6).map(|_| Dice::from_dice(&self)).collect::<Vec<Dice>>();
        
        // 異なる上面6パターンを得る
        v[1].move_e();
        v[2].move_w();
        v[3].move_n();
        v[4].move_s();
        v[5].move_s(); v[5].move_s();

        for d in v.iter_mut() {
            // 横回転4パターンで全てを探索
            for _ in 0..4 {
                if d.u == dice.u && d.f == dice.f && d.r == dice.r 
                    && d.l == dice.l && d.b == dice.b && d.d == dice.d {
                    return true;
                }
                d.rotate_r();
            }
        }

        false
    }
}

fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();

    let n = l_iter.next().unwrap().unwrap().parse::<usize>().unwrap();
    let dices = (0..n).map(|_| {
            let v = l_iter.next().unwrap().unwrap().split_whitespace()
                .map(|v| v.parse::<u32>().unwrap() )
                .collect::<Vec<u32>>();
            Dice::from_vec(&v) })
        .collect::<Vec<Dice>>();

    let mut diff_all = true;
    for i in 0..n-1 {
        for j in i+1..n {
            if dices[i].is_same(&dices[j]){
                diff_all = false;
                break;
            }
        }
    }

    match diff_all {
        true => println!("Yes"),
        false => println!("No"),
    };
}
