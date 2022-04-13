#![allow(non_snake_case, unused, dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn reverse_string(S: String) -> String {
    //! 文字列を反転する関数
    //! String型にreverseが実装されてないので自作
    S.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string(){
        let S = "ABC".to_string();
        assert_eq!(reverse_string(S), "CBA");
        let S = "de2Fgg0".to_string();
        assert_eq!(reverse_string(S), "0ggF2ed");
        let S = "".to_string();
        assert_eq!(reverse_string(S),"");
    }
}