#![allow(unused)]
use itertools::Itertools;
use my_lib::*;
use procon_input::*;
use std::{
    cell::RefCell,
    clone,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    iter::FromIterator,
    ops::*,
    rc::Rc,
    slice::SliceIndex,
};

// These crates can't be used in Paiza, but in AtCoder.
// use rand::prelude::*;
// use rand_pcg::Mcg128Xsl64;
// use itertools::Itertools;
// use num::{integer::Roots, Integer, ToPrimitive};
// use proconio::{
//     fastout, input,
//     marker::{Bytes, Chars},
// };
// use superslice::Ext;

fn main() {
    let (n, m) = read_uu();
    let mut uf = Dsu::new(n);
    for m in 0..m {
        let (u, v) = read_uu();

        uf.merge(u - 1, v - 1);
    }
    println!("{}", uf.group_num());
}

struct Dsu {
    parent_or_size: Vec<i64>, // 親のindex or 親のときはグループのサイズを-1した値(for 経路圧縮)
    num_node: usize,
    num_group: usize,

    // extentions
    min_index: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        let mut min_index = Vec::<usize>::new();
        for index in 0..n as usize {
            min_index.push(index);
        }

        Dsu {
            parent_or_size: vec![-1; n],
            num_node: n,
            num_group: n,
            min_index,
        }
    }

    pub fn leader(&mut self, index: usize) -> usize {
        //! 代表元のindex取得
        assert!(index < self.num_node);

        let parent_index = self.parent_or_size[index];
        if self.parent_or_size[index] < 0 {
            index
        } else {
            let parent_index = self.leader(parent_index as usize);
            self.parent_or_size[index] = parent_index as i64;
            parent_index
        }
    }

    pub fn leader_vec(&self) -> Vec<usize> {
        let mut leaders = Vec::new();
        for (index, size_minus) in self.parent_or_size.iter().enumerate() {
            if *size_minus < 0 {
                leaders.push(index as usize);
            }
        }
        leaders
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.num_node);
        assert!(b < self.num_node);

        let mut leader_a = self.leader(a);
        let mut leader_b = self.leader(b);

        // 既に同じグループ
        if leader_a == leader_b {
            return leader_a;
        }

        // グループのサイズが大きいほうにマージする
        // 代表元のparent_or_sizeにはグループのサイズに-1した値が格納されている
        let group_size_a = -self.parent_or_size[leader_a];
        let group_size_b = -self.parent_or_size[leader_b];
        // aを基準にする
        if group_size_a < group_size_b {
            std::mem::swap(&mut leader_a, &mut leader_b);
        }
        // サイズ加算
        self.parent_or_size[leader_a] += self.parent_or_size[leader_b];
        self.parent_or_size[leader_b] = leader_a as i64;

        // グループ統合により、グループ数が減る
        self.num_group -= 1;

        // グループの最小index更新
        if self.min_index[leader_a] > self.min_index[leader_b] {
            self.min_index[leader_a] = self.min_index[leader_b];
        }

        leader_a
    }

    pub fn is_same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.num_node);
        assert!(b < self.num_node);

        self.leader(a) == self.leader(b)
    }

    pub fn group_size(&mut self, leader: usize) -> usize {
        assert!(leader < self.num_node);

        (-self.parent_or_size[leader]) as usize
    }

    pub fn group_num(&mut self) -> usize {
        self.num_group
    }

    pub fn min_index(&mut self, leader: usize) -> usize {
        assert!(leader < self.num_node);

        self.min_index[leader]
    }
}

mod procon_input {
    use std::{any::type_name, io::*};

    fn read_block<T: std::str::FromStr>() -> T {
        let mut s = String::new();
        let mut buf = [0];
        loop {
            stdin().read(&mut buf).expect("can't read.");
            let c = buf[0] as char;
            if c == ' ' {
                break;
            }
            // for Linux
            if c == '\n' {
                break;
            }
            // for Windows
            if c == '\r' {
                // pop LR(line feed)
                stdin().read(&mut buf).expect("can't read.");
                break;
            }
            s.push(c);
        }
        s.parse::<T>()
            .unwrap_or_else(|_| panic!("can't parse '{}' to {}", s, type_name::<T>()))
    }

    pub fn read_i() -> i64 {
        read_block::<i64>()
    }

    pub fn read_ii() -> (i64, i64) {
        (read_block::<i64>(), read_block::<i64>())
    }

    pub fn read_iii() -> (i64, i64, i64) {
        (
            read_block::<i64>(),
            read_block::<i64>(),
            read_block::<i64>(),
        )
    }

    pub fn read_iiii() -> (i64, i64, i64, i64) {
        (
            read_block::<i64>(),
            read_block::<i64>(),
            read_block::<i64>(),
            read_block::<i64>(),
        )
    }

    pub fn read_u() -> usize {
        read_block::<usize>()
    }

    pub fn read_uu() -> (usize, usize) {
        (read_block::<usize>(), read_block::<usize>())
    }

    pub fn read_uuu() -> (usize, usize, usize) {
        (
            read_block::<usize>(),
            read_block::<usize>(),
            read_block::<usize>(),
        )
    }

    pub fn read_uuuu() -> (usize, usize, usize, usize) {
        (
            read_block::<usize>(),
            read_block::<usize>(),
            read_block::<usize>(),
            read_block::<usize>(),
        )
    }

    pub fn read_f() -> f64 {
        read_block::<f64>()
    }

    pub fn read_ff() -> (f64, f64) {
        (read_block::<f64>(), read_block::<f64>())
    }

    pub fn read_c() -> char {
        read_block::<char>()
    }

    pub fn read_cc() -> (char, char) {
        (read_block::<char>(), read_block::<char>())
    }

    fn read_line() -> String {
        let mut s = String::new();
        stdin().read_line(&mut s).expect("can't read.");
        s.trim()
            .parse()
            .unwrap_or_else(|_| panic!("can't trim in read_line()"))
    }

    pub fn read_vec<T: std::str::FromStr>() -> Vec<T> {
        read_line()
            .split_whitespace()
            .map(|e| {
                e.parse()
                    .unwrap_or_else(|_| panic!("can't parse '{}' to {}", e, type_name::<T>()))
            })
            .collect()
    }

    pub fn read_i_vec() -> Vec<i64> {
        read_line()
            .split_whitespace()
            .map(|e| {
                e.parse()
                    .unwrap_or_else(|_| panic!("can't parse '{}' to {}", e, type_name::<i64>()))
            })
            .collect()
    }

    pub fn read_u_vec() -> Vec<usize> {
        read_line()
            .split_whitespace()
            .map(|e| {
                e.parse()
                    .unwrap_or_else(|_| panic!("can't parse '{}' to {}", e, type_name::<usize>()))
            })
            .collect()
    }

    pub fn read_f_vec() -> Vec<f64> {
        read_line()
            .split_whitespace()
            .map(|e| {
                e.parse()
                    .unwrap_or_else(|_| panic!("can't parse '{}' to {}", e, type_name::<f64>()))
            })
            .collect()
    }

    pub fn read_c_vec() -> Vec<char> {
        read_line()
            .split_whitespace()
            .map(|e| {
                e.parse()
                    .unwrap_or_else(|_| panic!("can't parse '{}' to {}", e, type_name::<char>()))
            })
            .collect()
    }

    pub fn read_line_as_chars() -> Vec<char> {
        read_line().as_bytes().iter().map(|&b| b as char).collect()
    }

    pub fn read_string() -> String {
        read_block()
    }
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

    pub fn pow_mod(a: usize, b: usize, p: usize) -> usize {
        //! O(log(b))
        if b == 0 {
            1
        } else if b.is_even() {
            let d = pow_mod(a, b / 2, p);
            (d * d) % p
        } else {
            (a * pow_mod(a, b - 1, p)) % p
        }
    }

    pub fn pow_non_mod(a: usize, b: usize) -> usize {
        //! O(log(b))
        if b == 0 {
            1
        } else if b.is_even() {
            let d = pow_non_mod(a, b / 2);
            (d * d)
        } else {
            (a * pow_non_mod(a, b - 1))
        }
    }

    pub const DXY: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
}
