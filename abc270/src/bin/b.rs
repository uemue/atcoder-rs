#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    }

    // straight forward
    if x * y < 0 {
        println!("{}", x.abs());
        return;
    }

    if x.abs() < y.abs() {
        println!("{}", x.abs());
        return;
    }

    // straight forward getting hammer
    if y * z > 0 && y.abs() > z.abs() {
        println!("{}", x.abs());
        return;
    }

    // go back and get hammer
    if y * z < 0 {
        println!("{}", z.abs() * 2 + x.abs());
        return;
    }

    println!("-1");
}
