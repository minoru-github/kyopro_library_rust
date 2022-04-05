#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn binary_search(n: i64) -> i64 {
    let is_ok = |x: i64| -> bool {
        // 満たすべき条件
        if x >= (n * n + 2 * n) {
            true
        } else {
            false
        }
    };

    let mut ok = 1000i64; // 満たす中でのでかい数
    let mut ng = -1; // ギリギリ満たさない数
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(1), 3);
        assert_eq!(binary_search(22), 528);
    }
}
