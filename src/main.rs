#![allow(non_snake_case, unused)]
use num::{integer::Roots, Integer};
use proconio::{input, marker::Chars};
use rand::prelude::*;
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    iter::FromIterator,
    ops::Range,
    ops::*,
    time,
};
use superslice::Ext;

macro_rules! p {
    ($x:expr) => {
        println!("{}", $x);
    };
}

macro_rules! p2 {
    ($x:expr, $y:expr) => {
        println!("{} {}", $x, $y);
    };
}

macro_rules! d {
    ($x:expr) => {
        println! {"{:?}", $x};
    };
}

fn chmin<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}

fn chmax<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a < b {
        *a = b;
        true
    } else {
        false
    }
}

fn main() {
    // a : 型
    // (a,b) : (型, 型)
    // a_vec : [型;サイズ]
    // a_vec2 : [[型;サイズ];サイズ]
    // S : Chars
    input! {
        N:usize,
    };
}
