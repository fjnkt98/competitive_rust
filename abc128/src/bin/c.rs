use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(n: usize, m: usize, s: Vec<Vec<usize>>, p: Vec<i64>) -> Result<()> {
    let mut count = 0;
    let n: i64 = n as i64;
    for bit in 0..(1 << n) {
        let mut turned: usize = 0;
        for (i, s) in s.iter().cloned().enumerate() {
            let mut sum = 0;
            for s in s.iter() {
                if (bit >> (s - 1)) & 1 == 1 {
                    sum += 1;
                }
            }
            if sum % 2 == p[i] {
                turned += 1
            }
        }
        if turned == m {
            count += 1;
        }
    }
    println!("{}", count);

    return Ok(());
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [[usize]; m],
        p: [i64; m],
    };

    solve(n, m, s, p).unwrap();
}
