fn dag() {
    let (n, m) = read_uu();

    let mut graph = vec![vec![]; n];
    let mut indegree_vec = vec![0usize; n];
    for i in 0..m {
        let (a, b) = read_uu();
        graph[a].push(b);
        indegree_vec[b] += 1;
    }

    let mut que = VecDeque::new();
    for (id, &degree) in indegree_vec.iter().enumerate() {
        if degree == 0 {
            que.push_back(id);
        }
    }

    while !que.is_empty() {
        let cur_id = que.pop_front().unwrap();
        let childs = &graph[cur_id];

        for &child_id in childs {
            indegree_vec[child_id] -= 1;
            if indegree_vec[child_id] == 0 {
                que.push_back(child_id);
            }
        }
    }

    if indegree_vec.iter().all(|cnt| *cnt == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}

mod dfs {
    fn dfs() {
        let mut graph: Graph = vec![vec![]; n];
        let mut has_seen = vec![false; n];
        type Graph = Vec<Vec<usize>>;
        
        fn dfs(cur: usize, graph: &Graph, has_seen: &mut Vec<bool>) {
            // 行きがけ順はここで何か処理
        
            has_seen[cur] = true;
            let childs = graph.get(cur).unwrap();
            for &child in childs {
                if has_seen[child] {
                    continue;
                }
                dfs(child, graph, has_seen);
            }
            // 帰りがけ順はここで何か処理
        }
    }
}