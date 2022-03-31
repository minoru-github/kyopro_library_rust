#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn chmin<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}

#[snippet]
fn chmax<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a < b {
        *a = b;
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_chmin() {
        let mut a = 3;
        assert_eq!(chmin(&mut a, 1), true);
        assert_eq!(a, 1);

        let mut a = 3;
        assert_eq!(chmin(&mut a, 5), false);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_chmax() {
        let mut a = 3;
        assert_eq!(chmax(&mut a, 4), true);
        assert_eq!(a, 4);

        let mut a = 3;
        assert_eq!(chmax(&mut a, 1), false);
        assert_eq!(a, 3);
    }
}

