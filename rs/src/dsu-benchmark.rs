extern crate dibrova;
extern crate rand;

use dibrova::dsu::{ForestDsu, OptimizedForestDsu, DSU};
use rand::random;
use std::time::{Duration, Instant};

fn do_tasks<'a, D: DSU<'a, usize>>(
    tasks: &'a Vec<(char, &usize, &usize)>,
    dsu: &mut D,
) -> (Vec<bool>, Duration) {
    let mut res: Vec<bool> = vec![];
    let start = Instant::now();
    for (tp, arg1, arg2) in tasks {
        match tp {
            'j' => dsu.join(&arg1, &arg2),
            'q' => res.push(dsu.is_same_set(&arg1, &arg2)),
            _ => panic!("unexpected task"),
        }
    }
    (res, Instant::now().duration_since(start))
}

fn main() {
    let mut e = vec![];
    let n = 1_000_000;
    for i in 0..n {
        e.push(i);
    }

    let m = 100_000;
    let mut tasks: Vec<(char, &usize, &usize)> = vec![];
    for _ in 0..m {
        let tp = if random::<u8>() % 50 == 0 { 'j' } else { 'q' };
        let arg1 = random::<usize>() % n;
        let arg2 = random::<usize>() % n;
        tasks.push((tp, &e[arg1], &e[arg2]));
    }

    let mut od: OptimizedForestDsu<usize> = OptimizedForestDsu::new();
    let mut fd: ForestDsu<usize> = ForestDsu::new();

    for i in 0..n {
        od.insert(&e[i]);
        fd.insert(&e[i]);
    }

    let (replies_o, duration_o) = do_tasks(&tasks, &mut od);
    let (replies_f, duration_f) = do_tasks(&tasks, &mut fd);
    assert_eq!(replies_o, replies_f);
    println!("Computation with {} elements, {} random queries", n, m);
    println!("Optimized forest DSU: {:?}", duration_o);
    println!("Naive forest DSU:     {:?}", duration_f);
}
