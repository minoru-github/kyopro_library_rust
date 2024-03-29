#![allow(unused)]
use std::cmp::{max, min};

fn compute_combination_naively(n: usize, r: usize) -> usize {
    let n1: usize = n + 1;
    let r: usize = min(r, n - r);
    let mut nmrt: usize = 1;
    let mut dnmnt: usize = 1;
    for i in 1..=r {
        nmrt = nmrt * (n1 - i);
        dnmnt *= i;
    }
    nmrt / dnmnt
}

fn compute_combination(mut n: usize, r: usize) -> usize {
    let mut res = 1;
    for i in 1..=r {
        res *= n;
        res /= i;
        n -= 1;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_combination_naively() {
        let a = compute_combination_naively(10, 2);
        assert_eq!(a, 45);

        let a = compute_combination_naively(29, 15);
        assert_eq!(a, 77558760);

        // overflow case in naive case
        // let a = compute_combination_naively(39, 15);
        // assert_eq!(a, 25140840660);
    }

    #[test]
    fn test_compute_combination() {
        let a = compute_combination(10, 2);
        assert_eq!(a, 45);

        let a = compute_combination(29, 15);
        assert_eq!(a, 77558760);

        // overflow case in naive case
        let a = compute_combination(39, 15);
        assert_eq!(a, 25140840660);

        let a = compute_combination(60, 30);
        assert_eq!(a, 118264581564861424);
    }
}
