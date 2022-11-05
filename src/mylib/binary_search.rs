#![allow(dead_code)]

fn binary_search(target: usize, vec: &Vec<usize>) -> usize {
    // vec[]の中で、vec[index] < targetとなる最大のindexを探す。
    let is_ok = |index: usize| -> bool { vec[index] < target };
    let mut ok = 0; // 満たす中でとりうる値の限界
    let mut ng = vec.len() as i64; // ギリギリ満たさない数

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

#[cfg(test)]
mod test {
    #[test]
    fn test_func() {
        fn func(val: i64) -> i64 {
            val * val + 2 * val
        }

        fn binary_search(target: i64) -> i64 {
            // func(target)以上の最小値を探す
            let is_ok = |x: i64| -> bool { x >= func(target) };
            let mut ok = 1000i64; // 満たす中でとりうる値の限界
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
    fn test_array_1() {
        fn binary_search(target: usize, vec: &Vec<usize>) -> usize {
            // vec[]の中で、vec[index] < targetとなる最大のindexを探す。
            let is_ok = |index: usize| -> bool { vec[index] < target };
            let mut ok = 0; // 満たす中でとりうる値の限界
            let mut ng = vec.len() as i64; // ギリギリ満たさない数

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
        // [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
        let index = binary_search(5, &vec);
        assert_eq!(index, 2);
    }

    #[test]
    fn test_array_2() {
        fn binary_search(target: usize, vec: &Vec<usize>) -> usize {
            // vec[]の中で、vec[index] >= targetとなる最小のindexを探す。
            let is_ok = |index: usize| -> bool { vec[index] >= target };
            let mut ok = vec.len() as i64 - 1; // 満たす中でとりうる値の限界
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
        // [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
        let index = binary_search(5, &vec);
        assert_eq!(index, 3);
    }
}
