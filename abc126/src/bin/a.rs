use itertools::Itertools;
use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(_: i64, k: usize, s: String) -> Result<()> {
    let mut s = s.chars().collect_vec();
    s[k - 1] = s[k - 1].to_ascii_lowercase();

    println!("{}", s.iter().collect::<String>());
    return Ok(());
}

fn main() {
    input! {
        n: i64,
        k: usize,
        s: String
    };

    solve(n, k, s).unwrap();
}
