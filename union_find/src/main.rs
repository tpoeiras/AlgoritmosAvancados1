use itertools::Itertools;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::rngs::StdRng;

use rand::prelude::*;

use std::cmp::Ordering;
use std::time::Instant;

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).into_iter().collect::<Vec<_>>();

        UnionFind {
            parent,
            rank: vec![0; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }

        self.parent[i]
    }

    fn find_simple(&self, mut i: usize) -> usize {
        while self.parent[i] != i {
            i = self.parent[i]
        }

        i
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);

        match self.rank[i].cmp(&self.rank[j]) {
            Ordering::Less => {
                self.parent[i] = j;
            }
            Ordering::Equal => {
                self.parent[j] = i;
                self.rank[i] += 1;
            }
            Ordering::Greater => {
                self.parent[j] = i;
            }
        }
    }

    fn union_simple(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);

        self.parent[i] = j;
    }

    fn pretty_print(&mut self) {
        let n = self.parent.len();
        let mut sets = vec![vec![]; n];

        for i in 0..n {
            let rep = self.find(i);
            sets[rep].push(i);
        }

        for i in 0..n {
            if !sets[i].is_empty() {
                println! {"{{{}}}", sets[i].iter().join(",")}
            }
        }
    }
}

fn test_union_find(
    rng: &mut StdRng,
    size: usize,
    mut unions: usize,
    mut finds: usize,
    rank_heuristic: bool,
    path_compression: bool,
) -> Vec<u128> {
    let mut operations_times = Vec::new();
    let mut elapsed_time = 0;

    let between = Uniform::from(0..size);

    let mut counter = 0;
    let mut union_find = UnionFind::new(size);
    while unions + finds > 0 {
        let bernoulli = Bernoulli::new((unions as f64) / ((unions + finds) as f64)).unwrap();
        if bernoulli.sample(rng) {
            unions -= 1;
            let (val1, val2) = (between.sample(rng), between.sample(rng));

            let start = Instant::now();
            if rank_heuristic {
                union_find.union(val1, val2);
            } else {
                union_find.union_simple(val1, val2);
            }
            elapsed_time += start.elapsed().as_nanos();

            counter += 1;
            if counter == 10000 {
                operations_times.push(elapsed_time);
                counter = 0;
            }
        } else {
            finds -= 1;
            let val = between.sample(rng);

            let start = Instant::now();
            if path_compression {
                union_find.find(val);
            } else {
                union_find.find_simple(val);
            }
            elapsed_time += start.elapsed().as_nanos();

            counter += 1;
            if counter == 10000 {
                operations_times.push(elapsed_time);
                counter = 0;
            }
        }
    }

    operations_times
}

fn main() {
    let size = 10000000;
    let unions = 5000000;
    let finds = 5000000;
    let n_tests = 10;

    let mut rng = StdRng::seed_from_u64(131254153214);

    for _ in 0..n_tests {
        let operations_times = test_union_find(&mut rng, size, unions, finds, false, true);

        for (op, time) in operations_times.iter().enumerate() {
            println!("{},{time}", op + 1);
        }
    }
}
