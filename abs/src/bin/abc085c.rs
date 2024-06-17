use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(n: i64, y: i64) -> Result<()> {
    for i in 0..=n {
        for j in 0..=(n - i) {
            let k = n - i - j;
            if k < 0 {
                continue;
            }
            let sum = 10000 * i + 5000 * j + 1000 * k;
            if sum == y {
                println!("{} {} {}", i, j, k);
                return Ok(());
            }
        }
    }
    println!("-1 -1 -1");
    return Ok(());
}

fn main() {
    input! {
        n: i64,
        y: i64,
    };

    solve(n, y).unwrap();
}
