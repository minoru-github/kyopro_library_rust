#![allow(non_snake_case, unused)]
use cargo_snippet::snippet;
use std::ops::*;

// memo:
// '22.04.17時点のAtCoderはRust1.42で、const generics使えないためMODULOをconst化
// ほんとは以下みたいにして、type時にgenericsとして素数を指定したい
// struct Modint<const MODULO:usize>{val: usize,}
// (なんかの処理)
// type Mint = Modint<998244353>;

const MODULO: usize = 998244353;
//const MODULO:usize = 1000000007;
#[derive(Copy, Clone, PartialEq, Eq)]
struct Modint {
    val: usize,
}

impl Add for Modint {
    type Output = Modint;
    fn add(self, other: Self) -> Self::Output {
        let add_val = (self.val + other.val) % MODULO;
        Self::Output { val: add_val }
    }
}

impl AddAssign for Modint {
    fn add_assign(&mut self, other: Self) {
        self.val = self.val + other.val;
        self.val %= MODULO;
    }
}

impl Sub for Modint {
    type Output = Modint;
    fn sub(self, other: Self) -> Self::Output {
        let mut self_val = self.val;
        if self_val < other.val {
            self_val += MODULO;
        }
        let sub_val = (self_val - other.val) % MODULO;
        Self::Output { val: sub_val }
    }
}

impl SubAssign for Modint {
    fn sub_assign(&mut self, other: Self) {
        // if self.val < other.val {
        //     self.val += MODULO;
        // }
        self.val = self.val - other.val;
    }
}

impl Mul for Modint {
    type Output = Modint;
    fn mul(self, other: Self) -> Self::Output {
        let mul_val = (self.val * other.val) % MODULO;
        Self::Output { val: mul_val }
    }
}

impl Modint {
    pub fn new(n: usize) -> Self {
        Self { val: n % MODULO }
    }

    pub fn val(&self) -> usize {
        self.val % MODULO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_non_mod() {
        let x = Modint::new(1);
        let y = Modint::new(203);
        let ans = x + y;
        assert_eq!(ans.val(), 204);
    }

    #[test]
    fn test_add_mod998244353() {
        let x = Modint::new(998244350);
        let y = Modint::new(10);
        let ans = x + y;
        assert_eq!(ans.val(), 7);
    }

    #[test]
    fn test_add_assign_mod998244353() {
        let mut ans = Modint::new(998244350);
        let y = Modint::new(10);
        ans += y;
        assert_eq!(ans.val(), 7);
    }

    #[test]
    fn test_sub_non_mod() {
        let x = Modint::new(100);
        let y = Modint::new(1);
        let ans = x - y;
        assert_eq!(ans.val(), 99);
    }

    #[test]
    fn test_sub_mod998244353_under_zero() {
        let x = Modint::new(0);
        let y = Modint::new(10);
        let ans = x - y;
        assert_eq!(ans.val(), 998244343);
    }

    #[test]
    fn test_sub_assign_mod998244353() {
        let mut ans = Modint::new(111);
        let y = Modint::new(11);
        ans -= y;
        assert_eq!(ans.val(), 100);
    }

    #[test]
    fn test_sub_assign_mod998244353_under_zero() {
        let mut ans = Modint::new(0);
        let y = Modint::new(10);
        ans -= y;
        assert_eq!(ans.val(), 998244343);
    }

    #[test]
    fn test_mul_non_mod() {
        let x = Modint::new(4);
        let y = Modint::new(17);
        let ans = x * y;
        assert_eq!(ans.val(), 68);
    }

    #[test]
    fn test_mul_mod998244353() {
        let x = Modint::new(998244350);
        let y = Modint::new(21738);
        let ans = x * y;
        assert_eq!(ans.val(), 998179139);
    }
}
