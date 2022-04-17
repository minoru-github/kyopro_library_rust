#![allow(non_snake_case, unused)]
use cargo_snippet::snippet;
use std::ops::*;

struct Modint<const MODULO: usize> {
    val: usize,
}

impl<const MODULO: usize> Add for Modint<MODULO> {
    type Output = Modint<MODULO>;
    fn add(self, other: Self) -> Self::Output {
        let add_val = (self.val + other.val) % MODULO;
        Self::Output { val: add_val }
    }
}

impl<const MODULO: usize> Sub for Modint<MODULO> {
    type Output = Modint<MODULO>;
    fn sub(self, other: Self) -> Self::Output {
        let mut self_val = self.val;
        if self_val < other.val {
            self_val += MODULO;
        }
        let sub_val = (self_val - other.val) % MODULO;
        Self::Output { val: sub_val }
    }
}

impl<const MODULO: usize> Mul for Modint<MODULO> {
    type Output = Modint<MODULO>;
    fn mul(self, other: Self) -> Self::Output {
        let mul_val = (self.val * other.val) % MODULO;
        Self::Output { val: mul_val }
    }
}

impl<const MODULO: usize> Modint<MODULO> {
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
        type Mint = Modint<1000000007>;
        let x = Mint::new(1);
        let y = Mint::new(203);
        let ans = x + y;
        assert_eq!(ans.val(), 204);
    }

    #[test]
    fn test_add_mod1000000007() {
        type Mint = Modint<1000000007>;
        let x = Mint::new(1000000000);
        let y = Mint::new(40000);
        let ans = x + y;
        assert_eq!(ans.val(), 39993);
    }

    #[test]
    fn test_add_mod998244353() {
        type Mint = Modint<998244353>;
        let x = Mint::new(998244350);
        let y = Mint::new(10);
        let ans = x + y;
        assert_eq!(ans.val(), 7);
    }

    #[test]
    fn test_sub_non_mod() {
        type Mint = Modint<1000000007>;
        let x = Mint::new(100);
        let y = Mint::new(1);
        let ans = x - y;
        assert_eq!(ans.val(), 99);
    }

    #[test]
    fn test_sub_mod1000000007() {
        type Mint = Modint<1000000007>;
        let x = Mint::new(1000000010);
        let y = Mint::new(1);
        let ans = x - y;
        assert_eq!(ans.val(), 2);
    }

    #[test]
    fn test_sub_mod1000000007_under_zero() {
        type Mint = Modint<1000000007>;
        let x = Mint::new(0);
        let y = Mint::new(10);
        let ans = x - y;
        assert_eq!(ans.val(), 999999997);
    }

    #[test]
    fn test_mul_non_mod() {
        type Mint = Modint<998244353>;
        let x = Mint::new(4);
        let y = Mint::new(17);
        let ans = x * y;
        assert_eq!(ans.val(), 68);
    }

    #[test]
    fn test_mul_mod998244353() {
        type Mint = Modint<998244353>;
        let x = Mint::new(998244350);
        let y = Mint::new(21738);
        let ans = x * y;
        assert_eq!(ans.val(), 998179139);
    }
}
