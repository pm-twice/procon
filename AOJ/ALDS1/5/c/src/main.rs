use std::io;
use std::f64;
use std::f64::consts;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
        }
    }

    /// 与えられた線分を三等分する二点s,tと、これを頂点とする正三角形(s,u,t)を計算する。
    /// この時、(s,u,t)を返す
    fn koch_curve(p1: &Point, p2: &Point) -> (Point, Point, Point) {
        let s = Point::new(
            (p1.x*2.0 + p2.x) / 3.0, 
            (p1.y*2.0 + p2.y) / 3.0);
        let t = Point::new(
            (p1.x + p2.x*2.0) / 3.0, 
            (p1.y + p2.y*2.0) / 3.0);
        let u = Point::triangle_point(&s, &t);

        (s,u,t)
    }

    // 与えられた2点から、正三角形の頂点を計算して返す関数
    fn triangle_point(p1: &Point, p2: &Point) -> Point {
        let r = ((p2.x-p1.x).powi(2) + (p2.y-p1.y).powi(2)).sqrt();
        let t = ((p2.y-p1.y)/(p2.x-p1.x)).atan();   // arctanからθを計算。
        
        // tは戻り値が-pi/2~pi/2。回転に際して分岐が必要。
        // case1: p1よりp2が右にある場合はt=-pi/2~pi/2となるので問題ない
        // case2: p1よりp2が左にある場合はt=pi/2~3pi/2となるとする。こうしないと三角形の残る頂点が逆向きになってしまう
        // おとなしく回転行列版使った方が楽だった気がする

        let t2 = if p2.x < p1.x {
            t + consts::PI/3.0 + consts::PI // 60°回転＋象限の補正
        } else {
            t + consts::PI/3.0  // 60°回転
        };

        let x = p1.x + r * t2.cos();
        let y = p1.y + r * t2.sin();
        Point::new(x,y)
    }

    fn print(&self) {
        println!("{:.8} {:.8}", self.x, self.y);
    }
}

/// n回の再帰によって、koch曲線の座標を出力する
fn koch_curve_rec(n: i32, p1: &Point, p2: &Point) {
    if n > 0 {
        let (s,u,t) = Point::koch_curve(&p1, &p2);
        koch_curve_rec(n-1, &p1, &s);
        s.print();
        koch_curve_rec(n-1, &s, &u);
        u.print();
        koch_curve_rec(n-1, &u, &t);
        t.print();
        koch_curve_rec(n-1, &t, &p2);
    }
}

fn koch_curve(n: i32) {
    let x = Point::new(0.0, 0.0);
    let y = Point::new(100.0, 0.0);
    x.print();
    koch_curve_rec(n, &x, &y);
    y.print();
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let n: i32 = buf.trim().parse().unwrap();
    koch_curve(n);
}
