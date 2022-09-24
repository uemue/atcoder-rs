#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s:[Chars;10]
    }

    let mut a = 11;
    let mut b = 0;
    let mut c = 11;
    let mut d = 0;

    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                a = min(a, i + 1);
                b = max(b, i + 1);
                c = min(c, j + 1);
                d = max(d, j + 1);
            }
        }
    }

    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
