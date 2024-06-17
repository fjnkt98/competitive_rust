use counter::Counter;
use proconio::{input, marker::Chars};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(s: Vec<char>) -> Result<()> {
    let counts = s.iter().collect::<Counter<_>>();
    println!("{}", counts[&'1']);
    return Ok(());
}

fn main() {
    input! {
        s: Chars,
    };

    solve(s).unwrap();
}
