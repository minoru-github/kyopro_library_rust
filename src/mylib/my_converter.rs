use std::collections::VecDeque;

trait MyConverter {
    fn convert_base_n_to_base_k(&self, base_n: usize, base_k: usize) -> String;
}

impl MyConverter for Vec<char> {
    fn convert_base_n_to_base_k(&self, base_n: usize, base_k: usize) -> String {
        let base_n_number_as_chars = self;
        let hex_offset = 'A';
        // まずはn進数を10進数にする
        let mut base_10_number = 0;
        for &c in base_n_number_as_chars {
            base_10_number *= base_n;
            let mut offset = '0' as u8;
            if c > '9' {
                offset = hex_offset as u8 - 10;
            }
            base_10_number += (c as u8 - offset) as usize;
        }

        // 10進数からk進数に変換
        let mut base_k_number_as_chars: VecDeque<char> = VecDeque::new();
        while base_10_number > 0 {
            let val = base_10_number % base_k;
            let mut offset = '0' as u8;
            if val > 9 {
                offset = hex_offset as u8 - 10;
            }
            let c = ((val as u8) + offset) as char;
            base_k_number_as_chars.push_front(c);
            base_10_number /= base_k;
        }

        let mut base_k_number_as_str = String::new();
        for c in base_k_number_as_chars {
            base_k_number_as_str.push(c);
        }

        if base_k_number_as_str.is_empty() {
            base_k_number_as_str.push('0');
        }

        base_k_number_as_str
    }
}

impl MyConverter for String {
    fn convert_base_n_to_base_k(&self, base_n: usize, base_k: usize) -> String {
        let base_n_number_as_str = self;
        let hex_offset = 'A';
        // まずはn進数を10進数にする
        let mut base_10_number = 0;
        for c in base_n_number_as_str.chars() {
            base_10_number *= base_n;
            let mut offset = '0' as u8;
            if c > '9' {
                offset = hex_offset as u8 - 10;
            }
            base_10_number += (c as u8 - offset) as usize;
        }

        // 10進数からk進数に変換
        let mut base_k_number_as_chars: VecDeque<char> = VecDeque::new();
        while base_10_number > 0 {
            let val = base_10_number % base_k;
            let mut offset = '0' as u8;
            if val > 9 {
                offset = hex_offset as u8 - 10;
            }
            let c = ((val as u8) + offset) as char;
            base_k_number_as_chars.push_front(c);
            base_10_number /= base_k;
        }

        let mut base_k_number_as_str = String::new();
        for c in base_k_number_as_chars {
            base_k_number_as_str.push(c);
        }

        if base_k_number_as_str.is_empty() {
            base_k_number_as_str.push('0');
        }

        base_k_number_as_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_base_2_to_base_k() {
        let s = "001011110000".to_string();
        assert_eq!(s.convert_base_n_to_base_k(2, 3), String::from("1000212"));
        assert_eq!(s.convert_base_n_to_base_k(2, 5), String::from("11002"));
        assert_eq!(s.convert_base_n_to_base_k(2, 7), String::from("2123"));
        assert_eq!(s.convert_base_n_to_base_k(2, 8), String::from("1360"));
        assert_eq!(s.convert_base_n_to_base_k(2, 10), String::from("752"));
        assert_eq!(s.convert_base_n_to_base_k(2, 12), String::from("528"));
        assert_eq!(s.convert_base_n_to_base_k(2, 16), String::from("2F0"));
        assert_eq!(s.convert_base_n_to_base_k(2, 20), String::from("1HC"));

        let s = "001011110000".to_string().chars().collect::<Vec<char>>();
        assert_eq!(s.convert_base_n_to_base_k(2, 3), String::from("1000212"));
        assert_eq!(s.convert_base_n_to_base_k(2, 5), String::from("11002"));
        assert_eq!(s.convert_base_n_to_base_k(2, 7), String::from("2123"));
        assert_eq!(s.convert_base_n_to_base_k(2, 8), String::from("1360"));
        assert_eq!(s.convert_base_n_to_base_k(2, 10), String::from("752"));
        assert_eq!(s.convert_base_n_to_base_k(2, 12), String::from("528"));
        assert_eq!(s.convert_base_n_to_base_k(2, 16), String::from("2F0"));
        assert_eq!(s.convert_base_n_to_base_k(2, 20), String::from("1HC"));
    }

    #[test]
    fn test_convert_base_12_to_base_k() {
        let s = "6A7".to_string();
        assert_eq!(s.convert_base_n_to_base_k(12, 2), String::from("1111011111"));
        assert_eq!(s.convert_base_n_to_base_k(12, 3), String::from("1100201"));
        assert_eq!(s.convert_base_n_to_base_k(12, 5), String::from("12431"));
        assert_eq!(s.convert_base_n_to_base_k(12, 7), String::from("2614"));
        assert_eq!(s.convert_base_n_to_base_k(12, 8), String::from("1737"));
        assert_eq!(s.convert_base_n_to_base_k(12, 10), String::from("991"));
        assert_eq!(s.convert_base_n_to_base_k(12, 16), String::from("3DF"));
        assert_eq!(s.convert_base_n_to_base_k(12, 20), String::from("29B"));

        let s = "6A7".to_string().chars().collect::<Vec<char>>();
        assert_eq!(s.convert_base_n_to_base_k(12, 2), String::from("1111011111"));
        assert_eq!(s.convert_base_n_to_base_k(12, 3), String::from("1100201"));
        assert_eq!(s.convert_base_n_to_base_k(12, 5), String::from("12431"));
        assert_eq!(s.convert_base_n_to_base_k(12, 7), String::from("2614"));
        assert_eq!(s.convert_base_n_to_base_k(12, 8), String::from("1737"));
        assert_eq!(s.convert_base_n_to_base_k(12, 10), String::from("991"));
        assert_eq!(s.convert_base_n_to_base_k(12, 16), String::from("3DF"));
        assert_eq!(s.convert_base_n_to_base_k(12, 20), String::from("29B"));
    }
}
