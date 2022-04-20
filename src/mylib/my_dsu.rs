#![allow(non_snake_case, unused, dead_code)]

struct Dsu {
    parent_or_size: Vec<i64>, // 親のindex or 代表元のときはグループのサイズ(for 経路圧縮)
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
            min_index: min_index,
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
        let group_size_a = -1 * self.parent_or_size[leader_a];
        let group_size_b = -1 * self.parent_or_size[leader_b];
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

        if self.leader(a) == self.leader(b) {
            true
        } else {
            false
        }
    }

    pub fn group_size(&mut self, a: usize) -> usize {
        assert!(a < self.num_node);

        (-1 * self.parent_or_size[a]) as usize
    }

    pub fn group_num(&mut self) -> usize {
        self.num_group
    }

    pub fn min_index(&mut self, leader: usize) -> usize {
        assert!(leader < self.num_node);
        assert!(self.parent_or_size[leader] < 0);

        self.min_index[leader]
    }
}
