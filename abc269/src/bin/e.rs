#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use std::ptr::read;
use superslice::*;
use text_io::read;

fn main() {
    let n: usize = read!();

    // detect X(row number)
    let mut ok = n;
    let mut ng = 0;

    while abs(ok, ng) > 1 {
        let mid = (ok + ng) / 2;
        if ask(1, mid, 1, n) < mid {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let x = ok;

    // detect Y(column number)
    let mut ok = n;
    let mut ng = 0;

    while abs(ok, ng) > 1 {
        let mid = (ok + ng) / 2;
        if ask(1, n, 1, mid) < mid {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let y = ok;

    println!("! {} {}", x, y);
}

fn ask(a: usize, b: usize, c: usize, d: usize) -> usize {
    println!("? {} {} {} {}", a, b, c, d);
    let res: usize = read!();
    res
}

fn abs(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
