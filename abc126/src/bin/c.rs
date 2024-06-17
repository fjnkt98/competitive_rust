use num::pow;
use num::{rational::Ratio, ToPrimitive};
use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(n: i64, k: i64) -> Result<()> {
    let mut res = Ratio::new(0, 1);

    for i in 1..=n {
        let mut p = i;
        let mut c = 0;
        while p < k {
            p *= 2;
            c += 1;
        }

        res += Ratio::new(1, n) * Ratio::new(1, pow(2, c));
    }

    println!("{}", res.to_f64().unwrap());
    return Ok(());
}

fn main() {
    input! {
        n: i64,
        k: i64,
    };

    solve(n, k).unwrap();
}
