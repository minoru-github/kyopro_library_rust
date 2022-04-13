#![allow(non_snake_case, unused, dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn reverse_string(S: String) -> String {
    //! 文字列を反転する関数
    //! String型にreverseが実装されてないので自作
    S.chars().rev().collect()
}

#[snippet]
fn reverse_chars(mut S: Vec<char>) -> Vec<char> {
    //! 文字列を反転する関数
    S.reverse();
    S
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let S = "ABC".to_string();
        assert_eq!(reverse_string(S), "CBA");
        let S = "de2Fgg0".to_string();
        assert_eq!(reverse_string(S), "0ggF2ed");
        let S = "".to_string();
        assert_eq!(reverse_string(S), "");
    }

    #[test]
    fn test_reverse_chars() {
        let S = vec!['a', 'b', 'c'];
        assert_eq!(reverse_chars(S), ['c', 'b', 'a']);
        let S = vec!['a', '2', 'A', ' '];
        assert_eq!(reverse_chars(S), [' ', 'A', '2', 'a']);
        let S = vec!['a', 'b', 'a'];
        assert_eq!(reverse_chars(S.clone()), S);
    }
}
