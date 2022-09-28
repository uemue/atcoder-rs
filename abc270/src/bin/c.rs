#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
        edges: [(Usize1, Usize1); n-1]
    }

    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];

    for (from, to) in edges {
        tree[from].push(to);
        tree[to].push(from);
    }

    let mut seen: Vec<bool> = vec![false; n];
    let mut route: Vec<usize> = vec![];

    dfs(x, y, &tree, &mut seen, &mut route);
    route.reverse();

    print!("{}", x + 1);
    for i in route {
        print!(" {}", i + 1);
    }
}

fn dfs(
    start: usize,
    goal: usize,
    tree: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    route: &mut Vec<usize>,
) -> bool {
    seen[start] = true;
    if start == goal {
        return true;
    }

    for &next in &tree[start] {
        if seen[next] {
            continue;
        }
        let res = dfs(next, goal, tree, seen, route);
        if res {
            route.push(next);
            return true;
        }
    }
    return false;
}
