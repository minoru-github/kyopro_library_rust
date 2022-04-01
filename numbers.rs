#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use cargo_snippet::snippet;
use num::{integer::Roots, Integer};

#[snippet]
fn is_prime(x: u64) -> bool {
    if x < 2 {
        false
    } else {
        let last = x.sqrt();
        (2..=last).all(|n| x % n != 0)
    }
}

#[snippet]
fn prime_factorization(mut x: u64) -> Vec<(u64, u64)> {
    assert!(x != 0);

    let mut ret: Vec<(u64, u64)> = Vec::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prime() {
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
}
