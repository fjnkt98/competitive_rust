use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(n: i64, a: i64, b: i64) -> Result<()> {
    let mut count = 0;
    for i in 1..=n {
        let sum: i64 = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>()
            .into();
        if a <= sum && sum <= b {
            count += i;
        }
    }

    println!("{}", count);
    return Ok(());
}

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    };

    solve(n, a, b).unwrap();
}
