use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
struct Dijkstra {
    graph: Vec<Vec<(usize, i64)>>,
    distance: Vec<i64>,
    previous: Vec<Option<usize>>,
}

#[allow(dead_code)]
impl Dijkstra {
    fn new(graph: Vec<Vec<(usize, i64)>>) -> Self {
        let n = graph.len();
        return Self {
            graph: graph,
            distance: vec![1i64 << 60; n],
            previous: vec![None; n],
        };
    }

    fn search(&mut self, start: usize) -> Vec<i64> {
        let mut candidate: VecDeque<(i64, usize)> = VecDeque::with_capacity(self.graph.len());

        candidate.push_back((0, start));
        self.distance[start] = 0;

        while let Some((d, node)) = candidate.pop_front() {
            if d > self.distance[node] {
                continue;
            }

            for (next_node, weight) in self.graph[node].iter() {
                let d = self.distance[node] + weight;
                if self.distance[*next_node] > d {
                    self.distance[*next_node] = d;
                    self.previous[*next_node] = Some(node);
                    candidate.push_back((d, *next_node));
                }
            }
        }

        return self.distance.clone();
    }

    fn restore(&mut self, end: usize) -> Vec<usize> {
        let mut path = vec![end];
        let mut node = end;
        while let Some(n) = self.previous[node] {
            path.push(n);
            node = n;
        }

        return path.iter().cloned().rev().collect_vec();
    }
}

fn solve(n: usize, e: Vec<(usize, usize, i64)>) -> Result<()> {
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for (u, v, w) in e.iter() {
        let u = u - 1;
        let v = v - 1;
        graph[u].push((v, *w));
        graph[v].push((u, *w));
    }

    let mut dijkstra = Dijkstra::new(graph);
    let distance = dijkstra.search(0);
    let mut result = vec![0; n];
    for (i, d) in distance.iter().enumerate() {
        result[i] = d & 1;
    }
    println!("{}", result.iter().map(|r| r.to_string()).join("\n"));

    return Ok(());
}

fn main() {
    input! {
        n: usize,
        e: [(usize, usize, i64); n - 1]
    };

    solve(n, e).unwrap();
}
