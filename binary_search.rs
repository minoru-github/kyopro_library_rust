#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn binary_search() -> i64 {
    let is_ok = |x: i64| -> bool {
        // 満たすべき条件
        if x > 0 {
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
