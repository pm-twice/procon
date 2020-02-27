use std::io;

fn counting_sort(src: &Vec<u32>, dst: &mut Vec<u32>, k: u32) {
    let l = (k+1) as usize;
    let mut cnt: Vec<u32> = vec![0; l];

    for e in src.iter() {
        cnt[*e as usize] += 1;
    }

    // 累積和に変更
    for i in 1..l {
        cnt[i] = cnt[i] + cnt[i-1];
    }

    for v in src.iter().rev() {
        // 後ろから行うことで安定になる
        let i = cnt[*v as usize] - 1;
        dst[i as usize] = *v;
        cnt[*v as usize] -= 1;
    }
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let n: usize = buf.trim().parse().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let src: Vec<u32> = buf.split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut dst = vec![0; n];
    let k = src.iter().max().unwrap();

    counting_sort(&src, &mut dst, *k);

    let out = dst.iter().map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);
}
