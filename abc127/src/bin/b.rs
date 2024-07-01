use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(r: i64, d: i64, x: i64) -> Result<()> {
    let mut xi = x;
    for _ in 1..=10 {
        xi = r * xi - d;
        println!("{}", xi);
    }
    return Ok(());
}

fn main() {
    input! {
        r: i64,
        d: i64,
        x: i64,
    };

    solve(r, d, x).unwrap();
}
