#![allow(dead_code, unused)]
use cargo_snippet::snippet;

trait ChangeMinMax {
    fn chmin(&mut self, rhs: Self) -> bool
    where
        Self: PartialOrd<Self> + Copy,
    {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }

    fn chmax(&mut self, rhs: Self) -> bool
    where
        Self: PartialOrd<Self> + Copy,
    {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}

impl<T> ChangeMinMax for T {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chmin() {
        let mut a = 3;
        assert_eq!(a.chmin(1), true);
        assert_eq!(a, 1);

        let mut a = 3;
        assert_eq!(a.chmin(5), false);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_chmax() {
        let mut a = 3;
        assert_eq!(a.chmax(4), true);
        assert_eq!(a, 4);

        let mut a = 3;
        assert_eq!(a.chmax(1), false);
        assert_eq!(a, 3);
    }
}
