#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut n: usize
    }

    let mut mask = 1;
    let mut results: Vec<usize> = vec![0];
    println!("{}", 0);

    for _ in 0..60 {
        if n & mask > 0 {
            let mut new_results: Vec<usize> = Vec::new();
            for res in &results {
                let new_res = mask | res;
                println!("{}", new_res);
                new_results.push(new_res);
            }
            results.append(&mut new_results);
        }
        mask <<= 1;
    }
}
