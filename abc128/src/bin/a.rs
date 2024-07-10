use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(a: i64, p: i64) -> Result<()> {
    println!("{}", (a * 3 + p) / 2);
    return Ok(());
}

fn main() {
    input! {
        a: i64,
        p: i64
    };

    solve(a, p).unwrap();
}
