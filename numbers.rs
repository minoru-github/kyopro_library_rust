#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn is_prime(x: usize) -> bool {
    if x < 2 {
        false
    } else {
        let mut ret = true;
        for n in 2..=num::integer::sqrt(x) {
            if x % n == 0 {
                ret = false;
                break;
            }
        }

        ret
    }
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
}
