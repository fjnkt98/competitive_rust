use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(a: i64, b: i64) -> Result<()> {
    if a >= 13 {
        println!("{}", b);
    } else if a >= 6 {
        println!("{}", b / 2);
    } else {
        println!("0");
    }
    return Ok(());
}

fn main() {
    input! {
        a: i64,
        b: i64,
    };

    solve(a, b).unwrap();
}
