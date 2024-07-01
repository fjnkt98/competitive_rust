use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(n: usize, m: i64, lr: Vec<(usize, usize)>) -> Result<()> {
    let mut cards = vec![0i64; n + 1];
    for (l, r) in lr.iter() {
        cards[*l - 1] += 1;
        cards[*r] -= 1;
    }

    for i in 1..=n {
        cards[i] += cards[i - 1];
    }

    println!("{}", cards.iter().filter(|&e| *e == m).count());
    return Ok(());
}

fn main() {
    input! {
        n: usize,
        m: i64,
        lr: [(usize, usize); m],
    };

    solve(n, m, lr).unwrap();
}
