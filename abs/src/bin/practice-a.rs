use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(a: i64, b: i64, c: i64, s: String) -> Result<()> {
    println!("{} {}", a + b + c, s);
    return Ok(());
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        s: String,
    };

    solve(a, b, c, s).unwrap();
}
