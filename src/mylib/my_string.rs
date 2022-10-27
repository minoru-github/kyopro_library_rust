#![allow(unused, dead_code)]

trait Reverse {
    fn get_reversed(&self) -> Self;
}

impl Reverse for String {
    fn get_reversed(&self) -> String {
        //! 文字列を反転する関数
        //! String型にreverseが実装されてないので自作
        self.chars().rev().collect()
    }
}

impl Reverse for Vec<char> {
    fn get_reversed(&self) -> Vec<char> {
        let mut s = self.clone();
        s.reverse();
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let s = "ABC".to_string();
        assert_eq!(s.get_reversed(), "CBA");
        let s = "de2Fgg0".to_string();
        assert_eq!(s.get_reversed(), "0ggF2ed");
        let s = "".to_string();
        assert_eq!(s.get_reversed(), "");
    }

    #[test]
    fn test_reverse_chars() {
        let mut s = vec!['a', 'b', 'c'];
        assert_eq!(s.get_reversed(), ['c', 'b', 'a']);
        let s = vec!['a', '2', 'A', ' '];
        assert_eq!(s.get_reversed(), [' ', 'A', '2', 'a']);
        let s = vec!['a', 'b', 'a'];
        assert_eq!(s.get_reversed().get_reversed(), s);
    }
}
