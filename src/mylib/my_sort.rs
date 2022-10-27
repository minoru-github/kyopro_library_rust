#![allow(unused)]

trait SortFloat {
    //! 浮動小数点としてNANが含まれないことを約束されている場合のsort処理
    fn sort(&mut self);
    fn sort_rev(&mut self);
}

impl SortFloat for Vec<f64> {
    fn sort(&mut self) {
        self.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    fn sort_rev(&mut self) {
        self.sort_by(|a, b| b.partial_cmp(a).unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_float() {
        let mut vec: Vec<f64> = vec![6.6, 1.2, 0.0, -4.99, 3.4, 5.5, 12345.99, -0.2];
        vec.sort();
        assert_eq!(vec, vec![-4.99, -0.2, 0.0, 1.2, 3.4, 5.5, 6.6, 12345.99]);
    }

    fn test_sort_rev_float() {
        let mut vec: Vec<f64> = vec![6.6, 1.2, 0.0, -4.99, 3.4, 5.5, 12345.99, -0.2];
        vec.sort_rev();
        assert_eq!(vec, vec![12345.99, 6.6, 5.5, 3.4, 1.2, 0.0, -0.2, -4.99]);
    }
}
