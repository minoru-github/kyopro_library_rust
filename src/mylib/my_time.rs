#![allow(dead_code, unused)]
use cargo_snippet::snippet;

fn update_time() -> f64 {
    static mut STARTING_TIME_MS: Option<f64> = None;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let time_ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        match STARTING_TIME_MS {
            Some(starting_time_ms) => time_ms - starting_time_ms,
            None => {
                STARTING_TIME_MS = Some(time_ms);
                0.0_f64
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::update_time;

    #[test]
    fn test_update_time() {
        let t1 = update_time();
        assert_eq!(t1, 0.0);
        for _ in 0..10000 {
            update_time();
        }
        let t2 = update_time();
        assert_ne!(t2, t1);
        assert!(t1 < t2);
    }
}
