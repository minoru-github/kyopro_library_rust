fn chmin<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}

fn chmax<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a < b {
        *a = b;
        true
    } else {
        false
    }
}