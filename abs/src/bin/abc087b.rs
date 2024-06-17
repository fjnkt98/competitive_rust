use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(a: i64, b: i64, c: i64, x: i64) -> Result<()> {
    let mut res = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
    return Ok(());
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        x: i64,
    };

    solve(a, b, c, x).unwrap();
}
