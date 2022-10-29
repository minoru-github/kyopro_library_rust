#![allow(unused)]

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    if a == 0 && b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        let a = 630;
        let b = 300;
        assert_eq!(gcd(a, b), 30);

        let a = 300;
        let b = 630;
        assert_eq!(gcd(a, b), 30);

        let a = 17;
        let b = 13;
        assert_eq!(gcd(a, b), 1);
    }

    #[test]
    fn test_lcm() {
        let a = 630;
        let b = 300;
        assert_eq!(lcm(a, b), 6300);

        let a = 300;
        let b = 630;
        assert_eq!(lcm(a, b), 6300);

        let a = 17;
        let b = 13;
        assert_eq!(lcm(a, b), 221);
    }
}
