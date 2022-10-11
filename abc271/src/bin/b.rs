#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        l: [[u64];n],
        k: [(Usize1, Usize1);q]
    }

    for (s, t) in k {
        println!("{}", l[s][t]);
    }
}
