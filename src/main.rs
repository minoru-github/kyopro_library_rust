#![allow(unused)]
use itertools::Itertools;
use num::{integer::Roots, Integer, ToPrimitive};
use procon_input::*;
use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;
use std::{
    clone,
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    iter::FromIterator,
    ops::Range,
    ops::*,
    slice::SliceIndex,
};
use superslice::Ext;

fn main() {
    
}

mod procon_input {
    //! This input module is written with reference to MoSoon.
    //! (https://atcoder.jp/users/MoSooN)
    //! Very, Very thank to MoSoon!
    use std::io::*;

    pub fn read<T: std::str::FromStr>() -> T {
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

    pub fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
        (0..n).map(|_| read_vec()).collect()
    }

    pub fn readi() -> (i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<i64>().unwrap()
    }

    pub fn readii() -> (i64, i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    }

    pub fn readiii() -> (i64, i64, i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    }

    pub fn readiiii() -> (i64, i64, i64, i64) {
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

    pub fn readu() -> (usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<usize>().unwrap()
    }

    pub fn readuu() -> (usize, usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    }

    pub fn readuuu() -> (usize, usize, usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    }

    pub fn readuuuu() -> (usize, usize, usize, usize) {
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

    pub fn readf() -> (f64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<f64>().unwrap()
    }

    pub fn readff() -> (f64, f64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
        )
    }

    pub fn readc() -> (char) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<char>().unwrap()
    }

    pub fn readcc() -> (char, char) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<char>().unwrap(),
            iter.next().unwrap().parse::<char>().unwrap(),
        )
    }

    pub fn readccc() -> (char, char, char) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
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

    pub fn read_string() -> String {
        read::<String>()
    }
}
