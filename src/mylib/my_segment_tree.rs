#![allow(non_snake_case, unused, dead_code)]

// references:
// https://github.com/hatoo/competitive-rust-snippets/tree/master/src
// https://algo-logic.info/segment-tree/#toc_id_2
trait Monoid {
    type T: Clone;
    fn e() -> Self::T;
    fn op(a: Self::T, b: Self::T) -> Self::T;
}

struct SUM;
impl Monoid for SUM {
    type T = u64;
    fn e() -> Self::T {
        0
    }
    fn op(a: Self::T, b: Self::T) -> Self::T {
        a + b
    }
}

struct MAX;
impl Monoid for MAX {
    type T = u64;
    fn e() -> Self::T {
        u64::min_value()
    }
    fn op(a: Self::T, b: Self::T) -> Self::T {
        a.max(b)
    }
}

struct SegmentTree<M: Monoid> {
    n: usize,
    query_end: usize,
    data: Vec<M::T>,
}

impl<M: Monoid> SegmentTree<M> {
    pub fn new(n: usize) -> Self {
        let mut adjusted_n = 1;
        while n > adjusted_n {
            adjusted_n *= 2;
        }
        SegmentTree {
            n: adjusted_n,
            query_end: n,
            data: vec![M::e(); 2 * adjusted_n],
        }
    }

    pub fn set(&mut self, idx: usize, a: M::T) {
        let mut node_idx = idx + self.n - 1;
        self.data[node_idx] = a;
        while node_idx > 0 {
            node_idx = (node_idx - 1) / 2;
            let child_1 = self.data[2 * node_idx + 1].clone();
            let child_2 = self.data[2 * node_idx + 2].clone();

            self.data[node_idx] = M::op(child_1, child_2);
        }
    }

    pub fn get(&self, idx: usize) -> M::T {
        if self.query_end < idx {
            panic!("segment_tree::get() : out of bounds");
        }
        let node_idx = idx + self.n - 1;
        self.data[node_idx].clone()
    }

    pub fn query(&self, left: usize, right: usize) -> M::T {
        if self.query_end < left {
            panic!("segment_tree::query() : left out of bounds");
        }
        if self.query_end < right {
            panic!("segment_tree::query() : right out of bounds");
        }
        if right <= left {
            panic!("segment_tree::query() : illegal range (right <= left)");
        }
        self.sub_query(left, right, 0, 0, self.n)
    }

    fn sub_query(
        &self,
        left: usize,
        right: usize,
        node_idx: usize,
        seg_begin: usize,
        seg_end: usize,
    ) -> M::T {
        if seg_end <= left || right <= seg_begin {
            M::e()
        } else if left <= seg_begin && seg_end <= right {
            self.data[node_idx].clone()
        } else {
            let child_idx_1 = 2 * node_idx + 1;
            let child_idx_2 = 2 * node_idx + 2;

            let mid = (seg_begin + seg_end) / 2;
            let child_1 = self.sub_query(left, right, child_idx_1, seg_begin, mid);
            let child_2 = self.sub_query(left, right, child_idx_2, mid, seg_end);
            M::op(child_1, child_2)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_sum_at_odd_size() {
        let n = 5;
        let mut st = SegmentTree::<SUM>::new(n);
        let input = vec![1, 4, 2, 3, 5];
        for (idx, a) in input.iter().enumerate() {
            st.set(idx, *a);
        }

        let ans = st.query(1, 2);
        assert_eq!(ans, 4);
        let ans = st.query(1, 5);
        assert_eq!(ans, 14);

        let mut expected = 0;
        for left in 0..n {
            let mut expected = input[left];
            for right in (left + 1)..n {
                let ans = st.query(left, right);
                if (left + 1) < right {
                    expected += input[right - 1];
                }
                assert_eq!(ans, expected);
            }
        }
    }

    #[test]
    fn test_range_sum_at_even_size() {
        let n = 6;
        let mut st = SegmentTree::<SUM>::new(n);
        let input = vec![1, 0, 2, 0, 5, 10];
        for (idx, a) in input.iter().enumerate() {
            st.set(idx, *a);
        }

        let ans = st.query(1, 2);
        assert_eq!(ans, 0);
        let ans = st.query(2, 6);
        assert_eq!(ans, 17);

        let mut expected = 0;
        for left in 0..n {
            let mut expected = input[left];
            for right in (left + 1)..n {
                let ans = st.query(left, right);
                if (left + 1) < right {
                    expected += input[right - 1];
                }
                assert_eq!(ans, expected);
            }
        }
    }

    #[test]
    fn test_range_max() {
        let N = 10;
        let input = vec![1, 53, 2, 44, 102, 15, 8, 9, 0];
        let mut st = SegmentTree::<MAX>::new(N);
        for (idx, x) in input.iter().enumerate() {
            st.set(idx, *x);
        }

        let ans = st.query(0, N);
        assert_eq!(ans, 102);

        let ans = st.query(5, 6);
        assert_eq!(ans, 15);

        let ans = st.query(1, 4);
        assert_eq!(ans, 53);
    }
}
