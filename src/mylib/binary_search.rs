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
    };

    let mut ok = 1000i64; // 満たす中での絶対値が大きい数
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
    #[test]
    fn test_lower_bound() {
        fn binary_search(value: i64) -> i64 {
            // (n * n + 2 * n)以上の最小値を探す
            let is_ok = |x: i64| -> bool { x >= (value * value + 2 * value) };

            let mut ok = 1000i64; // 満たす中での絶対値が大きい数
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

        assert_eq!(binary_search(1), 3);
        assert_eq!(binary_search(22), 528);
    }

    #[test]
    fn test_upper_bound() {
        fn binary_search(value: usize, vec: &Vec<usize>) -> usize {
            // vec[]の中で、value > vec[index]となる最小のindexを探す。
            let is_ok = |index: usize| -> bool { value > vec[index] };

            let mut ok = vec.len() as i64 - 1; // 満たす中での絶対値が大きい数
            let mut ng = -1; // ギリギリ満たさない数
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if is_ok(mid as usize) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok as usize
        }

        let vec: Vec<usize> = (0..=10).into_iter().map(|e| e * 2).collect();
        // TODO
        //let index = binary_search(5, &vec);
        //assert_eq!(index, 4);
    }
}
