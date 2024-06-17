use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(v: Vec<(i64, i64, i64)>) -> Result<()> {
    let mut t = 0;
    let mut x = 0;
    let mut y = 0;
    for (ti, xi, yi) in v.into_iter() {
        let distance = (xi - x).abs() + (yi - y).abs();
        let duration = ti - t;

        if distance <= duration && (duration - distance) & 1 == 0 {
            t = ti;
            x = xi;
            y = yi;
            continue;
        } else {
            println!("No");
            return Ok(());
        }
    }
    println!("Yes");
    return Ok(());
}

fn main() {
    input! {
        n: i64,
        v: [(i64, i64, i64); n],
    };

    solve(v).unwrap();
}
