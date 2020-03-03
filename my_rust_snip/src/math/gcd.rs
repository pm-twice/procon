use cargo_snippet::snippet;

/// ユークリッドの互除法による最大公約数の計算
/// 
/// スニペットは`gcd`で登録
#[snippet("gcd")]
pub fn gcd(a: u64, b: u64) -> u64 
{
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// ユークリッドの互除法による最小公倍数の計算
/// 
/// スニペットは`lcm`で登録。gcd依存なので同時に読み込まれる
#[snippet("lcm", include="gcd")]
pub fn lcm(a: u64, b: u64) -> u64 
{
    a / gcd(a, b) * b
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(57, 3), 3);
}

#[test]
fn test_lcm(){
    assert_eq!(lcm(6,8), 24);
}
