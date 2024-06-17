use itertools::Itertools;
use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(a: Vec<i64>) -> Result<()> {
    let a = a.iter().cloned().sorted_by(|a, b| b.cmp(a)).collect_vec();

    let alice = a.iter().step_by(2).sum::<i64>();
    let bob = a[1..].iter().step_by(2).sum::<i64>();
    println!("{}", alice - bob);
    return Ok(());
}

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    };

    solve(a).unwrap();
}
