use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(_: usize, _: usize, a: Vec<i64>, mut bc: Vec<(i64, i64)>) -> Result<()> {
    let mut heap = BinaryHeap::from(a.iter().cloned().map(|a| Reverse(a)).collect_vec());

    bc.sort_by(|a, b| b.cmp(&a));

    for (b, c) in bc.iter().cloned() {
        'inner: for _ in 0..b {
            if let Some(Reverse(h)) = heap.pop() {
                if h >= c {
                    heap.push(Reverse(h));
                    break 'inner;
                }
                heap.push(Reverse(c));
            }
        }
    }

    let sum = heap.iter().map(|h| h.0).sum::<i64>();
    println!("{}", sum);

    return Ok(());
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        bc: [(i64, i64); m],
    };

    solve(n, m, a, bc).unwrap();
}
