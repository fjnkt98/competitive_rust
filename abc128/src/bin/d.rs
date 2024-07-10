use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(n: usize, k: i64, v: Vec<i64>) -> Result<()> {
    let mut answer = 0;

    for l in 0..=n {
        for r in 0..=n {
            if n < l + r {
                continue;
            }
            if (k as usize) < l + r {
                continue;
            }

            let mut picked = [&v[0..l], &v[(n - r)..n]].concat();
            picked.sort_by(|a, b| b.cmp(a));

            let remain = k as usize - (l + r);
            for _ in 0..remain {
                if let Some(last) = picked.last() {
                    if *last < 0 {
                        picked.pop();
                    }
                }
            }

            let sum = picked.iter().sum();
            answer = std::cmp::max(answer, sum);
        }
    }

    println!("{}", answer);
    return Ok(());
}

fn main() {
    input! {
        n: usize,
        k: i64,
        v: [i64; n],
    };

    solve(n, k, v).unwrap();
}
