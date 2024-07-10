use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(_: usize, sp: Vec<(String, i64)>) -> Result<()> {
    let res = sp
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, (s, p))| (s, p, i + 1))
        .sorted_by_key(|(s, p, i)| (s.clone(), Reverse(*p), *i))
        .map(|(_, _, i)| i.to_string())
        .join("\n");

    println!("{}", res);

    return Ok(());
}
fn main() {
    input! {
        n: usize,
        sp: [(String, i64); n],
    };

    solve(n, sp).unwrap();
}
