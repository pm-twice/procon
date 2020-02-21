use std::io;

/// ベクトル1はleft...mid-1まで、ベクトル2はmid...right-1まで
/// 昇順に整列済みのベクトル1、ベクトル2を結合する。
fn merge(vec: &mut Vec<u32>, left: usize, mid:usize, right: usize) -> u32 {
    let n = right - left;
    let mut tv: Vec<u32> = vec![0; n];
    for i in 0..(mid-left) {
        tv[i] = vec[left+i];
    }
    for i in 0..(right-mid) {
        tv[n-1-i] = vec[mid+i];
    }

    let mut l = 0;
    let mut r = n - 1;
    let mut cnt = 0;
    for i in left..right {
        if tv[l] < tv[r] {
            vec[i] = tv[l];
            l += 1;
        } else {
            vec[i] = tv[r];
            if r > 0 {
                r -= 1;
            }
        }
        cnt +=1 ;
    }
    cnt
}

fn merge_sort(vec: &mut Vec<u32>, left: usize, right: usize) -> u32 {
    let mut cnt = 0;
    if right - left >= 2 {
        let mid = (left + right) / 2;
        cnt += merge_sort(vec, left, mid);
        cnt += merge_sort(vec, mid, right);
        cnt += merge(vec, left, mid, right);
    }
    cnt
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _n: usize = buf.trim().parse().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut s: Vec<u32> = buf.split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<u32>>();
    let size = s.len();
    let cnt = merge_sort(&mut s, 0, size);
    let out = s.iter().map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);
    println!("{}", cnt);
}
