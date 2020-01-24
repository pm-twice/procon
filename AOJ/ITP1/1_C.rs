use std::io;

fn main(){
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let (a, b) = {
        let mut ws = buf.split_whitespace();
        let a: i32 = ws.next().unwrap().parse().unwrap();
        let b: i32 = ws.next().unwrap().parse().unwrap();
        (a, b)
    };

    let area = a*b;
    let phi = 2*(a+b);

    println!("{} {}", area, phi);
}