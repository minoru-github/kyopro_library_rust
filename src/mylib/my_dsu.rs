#![allow(non_snake_case, unused, dead_code)]

struct Dsu {
    parent_or_size: Vec<i64>, // 親のindex or 親のときはグループのサイズを-1した値(for 経路圧縮)
    num_node: usize,
    num_group: usize,

    // extentions
    min_index: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        let mut min_index = Vec::<usize>::new();
        for index in 0..n as usize {
            min_index.push(index);
        }

        Dsu {
            parent_or_size: vec![-1; n],
            num_node: n,
            num_group: n,
            min_index,
        }
    }

    pub fn leader(&mut self, index: usize) -> usize {
        //! 代表元のindex取得
        assert!(index < self.num_node);

        let parent_index = self.parent_or_size[index];
        if self.parent_or_size[index] < 0 {
            index
        } else {
            let parent_index = self.leader(parent_index as usize);
            self.parent_or_size[index] = parent_index as i64;
            parent_index
        }
    }

    pub fn leader_vec(&self) -> Vec<usize> {
        let mut leaders = Vec::new();
        for (index, size_minus) in self.parent_or_size.iter().enumerate() {
            if *size_minus < 0 {
                leaders.push(index as usize);
            }
        }
        leaders
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.num_node);
        assert!(b < self.num_node);

        let mut leader_a = self.leader(a);
        let mut leader_b = self.leader(b);

        // 既に同じグループ
        if leader_a == leader_b {
            return leader_a;
        }

        // グループのサイズが大きいほうにマージする
        // 代表元のparent_or_sizeにはグループのサイズに-1した値が格納されている
        let group_size_a = -self.parent_or_size[leader_a];
        let group_size_b = -self.parent_or_size[leader_b];
        // aを基準にする
        if group_size_a < group_size_b {
            std::mem::swap(&mut leader_a, &mut leader_b);
        }
        // サイズ加算
        self.parent_or_size[leader_a] += self.parent_or_size[leader_b];
        self.parent_or_size[leader_b] = leader_a as i64;

        // グループ統合により、グループ数が減る
        self.num_group -= 1;

        // グループの最小index更新
        if self.min_index[leader_a] > self.min_index[leader_b] {
            self.min_index[leader_a] = self.min_index[leader_b];
        }

        leader_a
    }

    pub fn is_same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.num_node);
        assert!(b < self.num_node);

        self.leader(a) == self.leader(b)
    }

    pub fn group_size(&mut self, leader: usize) -> usize {
        assert!(leader < self.num_node);

        (-self.parent_or_size[leader]) as usize
    }

    pub fn group_num(&mut self) -> usize {
        self.num_group
    }

    pub fn min_index(&mut self, leader: usize) -> usize {
        assert!(leader < self.num_node);

        self.min_index[leader]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsu() {
        let n = 10;
        let mut uf = Dsu::new(n);

        struct Query {
            a: usize,
            b: usize,
        }

        let mut query = Vec::<Query>::new();
        query.push(Query { a: 0, b: 1 });
        query.push(Query { a: 1, b: 2 });
        query.push(Query { a: 0, b: 3 });
        query.push(Query { a: 4, b: 6 });
        query.push(Query { a: 3, b: 7 });
        query.push(Query { a: 8, b: 9 });

        for q in query {
            uf.merge(q.a, q.b);
        }

        assert_eq!(uf.group_num(), 4);

        assert_eq!(uf.leader(0), 0);
        assert_eq!(uf.leader(1), 0);
        assert_eq!(uf.leader(2), 0);
        assert_eq!(uf.leader(3), 0);
        assert_eq!(uf.leader(4), 4);
        assert_eq!(uf.leader(5), 5);
        assert_eq!(uf.leader(6), 4);
        assert_eq!(uf.leader(7), 0);
        assert_eq!(uf.leader(8), 8);
        assert_eq!(uf.leader(9), 8);

        assert_eq!(uf.leader_vec().len(), 4);
        assert_eq!(uf.leader_vec(), [0, 4, 5, 8]);

        assert_eq!(uf.is_same(0, 7), true);
        assert_eq!(uf.is_same(4, 9), false);

        let leader = uf.leader(3);
        assert_eq!(uf.min_index(leader), 0);
        let leader = uf.leader(5);
        assert_eq!(uf.min_index(leader), 5);

        let leader = uf.leader(3);
        assert_eq!(uf.group_size(leader), 5);
        let leader = uf.leader(5);
        assert_eq!(uf.group_size(leader), 1);
    }
}
