#![allow(non_snake_case)]

use std::io;

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
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let v = buf.split_whitespace()
        .map(|v| v.parse::<u32>().unwrap() )
        .collect::<Vec<u32>>();

    let mut dice = Dice::from_vec(&v);

    let mut ops = String::new();
    sin.read_line(&mut ops).ok();
    
    dice.operate_string(&ops);
    println!("{}", dice.top());
}
