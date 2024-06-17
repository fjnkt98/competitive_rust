use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(a: i64, b: i64) -> Result<()> {
    let prod = a * b;
    println!("{}", if (prod & 1) == 1 { "Odd" } else { "Even" });
    return Ok(());
}

fn main() {
    input! {
        a: i64,
        b: i64,
    };

    solve(a, b).unwrap();
}
