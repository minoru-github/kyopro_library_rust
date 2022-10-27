#![allow(unused)]

trait MyNum {
    fn sqrt(&self) -> usize;
    fn is_prime(&self) -> bool;
    fn eratos(&self) -> Vec<usize>;
    fn enumerate_divisors(&self) -> Vec<usize>;
    fn prime_factorization(&self) -> Vec<(usize, usize)>;
    fn count_digits(&self) -> usize;
}

impl MyNum for usize {
    fn sqrt(&self) -> usize {
        if *self == 0 {
            return 0;
        }
        let mut x = 1;
        loop {
            if (x + 1) * (x + 1) > *self {
                return x;
            } else {
                x += 1;
            }
        }
    }

    fn is_prime(&self) -> bool {
        //! 素数判定
        //! O(sqrt(N))
        let x = *self;
        if x < 2 {
            false
        } else {
            let last = x.sqrt();
            (2..=last).all(|n| x % n != 0)
        }
    }

    fn eratos(&self) -> Vec<usize> {
        //! n以下の素数列挙(エラトステネスの篩)
        //! O(N*sqrt(N))
        // 最初はすべてが素数候補
        let n = *self;
        let mut is_prime_vec = vec![true; n as usize + 1];
        let last = n.sqrt();
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

    fn enumerate_divisors(&self) -> Vec<usize> {
        //! 約数列挙
        let n = *self;
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

    fn prime_factorization(&self) -> Vec<(usize, usize)> {
        //! 素因数分解
        assert!(*self != 0);

        let mut x = *self;
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

    fn count_digits(&self) -> usize {
        //! 引数nの桁数を数える
        let mut n = *self;
        let mut ans = 1;
        while n > 0 {
            n /= 10;
            if n > 0 {
                ans += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() {
        let n = 100_usize;
        assert_eq!(n.sqrt(), 10);
        assert_eq!(99.sqrt(), 9);
        assert_eq!(256.sqrt(), 16);
        assert_eq!(1.sqrt(), 1);
        assert_eq!(0.sqrt(), 0);
    }

    #[test]
    fn test_is_prime() {
        //! 素数判定のテスト
        assert_eq!(3.is_prime(), true);
        assert_eq!(17.is_prime(), true);
        assert_eq!(200560490131.is_prime(), true);
        assert_eq!(1.is_prime(), false);
        assert_eq!(4.is_prime(), false);
        assert_eq!(12.is_prime(), false);
        assert_eq!(1565912117761.is_prime(), false);
    }

    #[test]
    fn test_prime_factorization() {
        //! 素因数分解のテスト
        let ans = 4.prime_factorization();
        assert_eq!(ans, [(2, 2),]);
        let ans = 189.prime_factorization();
        assert_eq!(ans, [(3, 3), (7, 1),]);
        let ans = 30.prime_factorization();
        assert_eq!(ans, [(2, 1), (3, 1), (5, 1),]);
        let ans = 1.prime_factorization();
        assert_eq!(ans, []);
        let p_ex = 1024.prime_factorization();
        for (p, ex) in p_ex {
            assert_eq!(p, 2);
            assert_eq!(ex, 10);
        }
    }

    #[test]
    fn test_eratos() {
        //! n以下の素数列挙のテスト
        assert_eq!(12.eratos(), [2, 3, 5, 7, 11]);
        assert_eq!(1.eratos(), []);
    }

    #[test]
    fn test_enumerate_divisors() {
        //! 約数列挙のテスト
        assert_eq!(12.enumerate_divisors(), [1, 2, 3, 4, 6, 12]);
        assert_eq!(7.enumerate_divisors(), [1, 7]);
        assert_eq!(0.enumerate_divisors(), []);
        assert_eq!(1.enumerate_divisors(), [1]);
    }

    #[test]
    fn test_count_digits() {
        //! 桁数確認のテスト
        assert_eq!(1234.count_digits(), 4);
        assert_eq!(0.count_digits(), 1);
        assert_eq!(1.count_digits(), 1);
        assert_eq!(10.count_digits(), 2);
        assert_eq!(18_000_000_000_000_000_000.count_digits(), 20);
    }
}
