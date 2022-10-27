#![allow(unused)]
use my_lib::*;
use procon_input::*;
use std::{
    clone,
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    iter::FromIterator,
    ops::Range,
    ops::*,
    slice::SliceIndex,
};

// These crates can't be used in Paiza, but in AtCoder.
// use rand::prelude::*;
// use rand_pcg::Mcg128Xsl64;
// use itertools::Itertools;
// use num::{integer::Roots, Integer, ToPrimitive};
// use proconio::{
//     input,
//     marker::{Bytes, Chars},
// };
// use superslice::Ext;

fn main() {
    
}

mod procon_input {
    //! This input module is written with reference to MoSoon.
    //! (https://atcoder.jp/users/MoSooN)
    //! Very, Very thank to MoSoon!
    use std::io::*;

    fn read<T: std::str::FromStr>() -> T {
        let mut s = String::new();
        stdin().read_line(&mut s).ok();
        s.trim().parse().ok().unwrap()
    }

    pub fn read_vec<T: std::str::FromStr>() -> Vec<T> {
        read::<String>()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect()
    }

    // pub fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    //     (0..n).map(|_| read_vec()).collect()
    // }

    pub fn read_i() -> (i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<i64>().unwrap()
    }

    pub fn read_ii() -> (i64, i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    }

    pub fn read_iii() -> (i64, i64, i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    }

    pub fn read_iiii() -> (i64, i64, i64, i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    }

    pub fn read_u() -> (usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<usize>().unwrap()
    }

    pub fn read_uu() -> (usize, usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    }

    pub fn read_uuu() -> (usize, usize, usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    }

    pub fn read_uuuu() -> (usize, usize, usize, usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    }

    pub fn read_f() -> (f64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<f64>().unwrap()
    }

    pub fn read_ff() -> (f64, f64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
        )
    }

    pub fn read_fff() -> (f64, f64, f64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
        )
    }

    pub fn read_ffff() -> (f64, f64, f64, f64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
        )
    }

    pub fn read_c() -> (char) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<char>().unwrap()
    }

    pub fn read_cc() -> (char, char) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<char>().unwrap(),
            iter.next().unwrap().parse::<char>().unwrap(),
        )
    }

    pub fn read_ccc() -> (char, char, char) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<char>().unwrap(),
            iter.next().unwrap().parse::<char>().unwrap(),
            iter.next().unwrap().parse::<char>().unwrap(),
        )
    }

    pub fn read_cccc() -> (char, char, char, char) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<char>().unwrap(),
            iter.next().unwrap().parse::<char>().unwrap(),
            iter.next().unwrap().parse::<char>().unwrap(),
            iter.next().unwrap().parse::<char>().unwrap(),
        )
    }

    pub fn read_chars() -> Vec<char> {
        let mut vec = Vec::new();
        read::<String>()
            .as_bytes()
            .iter()
            .for_each(|&b| vec.push(b as char));
        vec
    }

    // pub fn read_string() -> String {
    //     read::<String>()
    // }
}

mod my_lib {
    pub trait SortFloat {
        fn sort(&mut self);
        fn sort_rev(&mut self);
    }

    impl SortFloat for Vec<f64> {
        fn sort(&mut self) {
            //! 浮動小数点としてNANが含まれないことを約束されている場合のsort処理<br>
            //! 小さい順
            self.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        fn sort_rev(&mut self) {
            //! 浮動小数点としてNANが含まれないことを約束されている場合のsort処理<br>  
            //! 大きい順
            self.sort_by(|a, b| b.partial_cmp(a).unwrap());
        }
    }

    pub trait EvenOdd {
        fn is_even(&self) -> bool;
        fn is_odd(&self) -> bool;
    }

    impl EvenOdd for usize {
        fn is_even(&self) -> bool {
            self % 2 == 0
        }

        fn is_odd(&self) -> bool {
            self % 2 != 0
        }
    }
}
