#![allow(dead_code)]

fn binary_search(n: i64) -> i64 {
    //! 二分探索(めぐる式)
    let is_ok = |x: i64| -> bool {
        // (n * n + 2 * n)以上の最小値を探す
        x >= (n * n + 2 * n)

        // n以上の最小値を探す。ok は大きい数、ngは小さい数にする。
        // x >= n

        // n未満の最小値を探す。ok は小さい数、ngは大きい数にする。
        //x < n

        // if 満たすべき条件 {
        //     true
        // } else {
        //     false
        // }
    };

    let mut ok = 1000i64; // 満たす中での絶対値が大きい数
    let mut ng = -1;      // ギリギリ満たさない数
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
        //! 二分探索のテスト
        assert_eq!(binary_search(1), 3);
        assert_eq!(binary_search(22), 528);
    }
}
