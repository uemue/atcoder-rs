#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        black: [(i64, i64); n]
    }

    let black: HashSet<(i64, i64)> = black.into_iter().collect();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut result = 0;

    for cell in &black {
        if visited.contains(&cell) {
            continue;
        } else {
            result += 1;
            dfs(cell, &black, &mut visited);
        }
    }

    println!("{}", result);
}

fn dfs(cell: &(i64, i64), black: &HashSet<(i64, i64)>, visited: &mut HashSet<(i64, i64)>) {
    if visited.contains(&cell) {
        return;
    }

    let &(i, j) = cell;
    visited.insert((i, j));

    let next_cells = vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j),
        (i + 1, j + 1),
    ];

    for next_cell in next_cells {
        if !black.contains(&next_cell) {
            continue;
        }
        dfs(&next_cell, black, visited);
    }
}
