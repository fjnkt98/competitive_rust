use itertools::Itertools;
use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(d: Vec<i64>) -> Result<()> {
    println!("{}", d.iter().sorted().cloned().dedup().collect_vec().len());
    return Ok(());
}

fn main() {
    input! {
        n: i64,
        d: [i64; n],
    };

    solve(d).unwrap();
}
