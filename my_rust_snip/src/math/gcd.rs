use cargo_snippet::snippet;

/// ユークリッドの互除法による最大公約数の計算
/// 
/// # スニペット登録名
/// `snp-gcd`
/// 
/// # 利用ライブラリ
/// なし
/// 
#[snippet("snp-gcd")]
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
/// gcd依存なので同時に読み込まれる
/// 
/// # スニペット登録名
/// `snp-lcm`
/// 
/// # 利用ライブラリ
/// なし
/// 
#[snippet("snp-lcm", include="snp-gcd")]
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
