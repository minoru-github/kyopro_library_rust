#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use cargo_snippet::snippet;
use itertools::Itertools;
use num::{integer::Roots, Integer};

fn is_prime(x: usize) -> bool {
    //! 素数判定
    //! O(sqrt(N))
    if x < 2 {
        false
    } else {
        let last = x.sqrt();
        (2..=last).all(|n| x % n != 0)
    }
}

fn prime_factorization(mut x: usize) -> Vec<(usize, usize)> {
    //! 素因数分解
    assert!(x != 0);

    let mut ret: Vec<(usize, usize)> = Vec::new();
    let last = x.sqrt();
    for n in 2..=last {
        if x % n != 0 {
            continue;
        }
        let mut ex = 0;
        while x % n == 0 {
            ex += 1;
            x /= n;
        }
        ret.push((n, ex));
    }

    if x != 1 {
        ret.push((x, 1));
    }

    ret
}

fn eratos(n: usize) -> Vec<usize> {
    //! n以下の素数列挙(エラトステネスの篩)
    //! O(N*sqrt(N))
    // 最初はすべてが素数候補
    let mut is_prime_vec = vec![true; n as usize + 1];
    let last = n.sqrt() as usize;
    for i in 2..=last {
        // 素数の倍数を素数候補から外す
        if is_prime_vec[i] {
            let mut j = 2;
            while i * j <= n as usize {
                is_prime_vec[i * j] = false;
                j += 1;
            }
        }
    }

    let mut ans = Vec::new();
    for (index, is_prime) in is_prime_vec.iter().enumerate().skip(2) {
        if *is_prime {
            ans.push(index as usize)
        }
    }

    ans
}

fn enumerate_divisors(n: usize) -> Vec<usize> {
    //! 約数列挙
    let mut ans = Vec::new();
    let last = n.sqrt();
    for i in 1..=last {
        if n % i != 0 {
            continue;
        }
        ans.push(i);
        if i != n / i {
            ans.push(n / i);
        }
    }
    ans.sort();

    ans
}

fn count_digits(mut n: usize) -> usize {
    //! 引数nの桁数を数える
    let mut ans = 1;
    while n > 0 {
        n /= 10;
        if n > 0 {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prime() {
        //! 素数判定のテスト
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(200560490131), true);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(1565912117761), false);
    }

    #[test]
    fn test_prime_factorization() {
        //! 素因数分解のテスト
        let ans = prime_factorization(4);
        assert_eq!(ans, [(2, 2),]);
        let ans = prime_factorization(189);
        assert_eq!(ans, [(3, 3), (7, 1),]);
        let ans = prime_factorization(30);
        assert_eq!(ans, [(2, 1), (3, 1), (5, 1),]);
        let ans = prime_factorization(1);
        assert_eq!(ans, []);
        let p_ex = prime_factorization(1024);
        for (p, ex) in p_ex {
            assert_eq!(p, 2);
            assert_eq!(ex, 10);
        }
    }

    #[test]
    fn test_eratos() {
        //! n以下の素数列挙のテスト
        assert_eq!(eratos(12), [2, 3, 5, 7, 11]);
        assert_eq!(eratos(1), []);
    }

    #[test]
    fn test_enumerate_divisors() {
        //! 約数列挙のテスト
        assert_eq!(enumerate_divisors(12), [1, 2, 3, 4, 6, 12]);
        assert_eq!(enumerate_divisors(7), [1, 7]);
        assert_eq!(enumerate_divisors(0), []);
        assert_eq!(enumerate_divisors(1), [1]);
    }

    #[test]
    fn test_count_digits() {
        //! 桁数確認のテスト
        assert_eq!(count_digits(1234), 4);
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(18_000_000_000_000_000_000), 20);
    }
}
