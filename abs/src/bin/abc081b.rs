use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(_: i64, a: Vec<i64>) -> Result<()> {
    let res = a
        .iter()
        .map(|a| {
            let mut count = 0;
            let mut b: i64 = *a;
            while b & 1 == 0 {
                b /= 2;
                count += 1;
            }
            count
        })
        .min()
        .unwrap_or(0);

    println!("{}", res);
    return Ok(());
}

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    };

    solve(n, a).unwrap();
}
