use std::collections::VecDeque;

fn bfs(source_node: usize, sink_node: usize, parent: &mut Vec<i32>, graph: &Vec<Vec<usize>>, capacity: &Vec<Vec<i32>>) -> i32 {
    parent.fill(-1);
    parent[source_node] = -2;

    let mut q: VecDeque<(usize, i32)> = VecDeque::new();
    q.push_back((source_node, i32::MAX));

    while !q.is_empty() {
        let (current_node, flow) = q.pop_front().unwrap();

        for next in graph[current_node].iter() {
            if parent[*next] == -1 && capacity[current_node][*next] != 0 {
                parent[*next] = current_node as i32;

                let new_flow = flow.min(capacity[current_node][*next]);
                if *next == sink_node {
                    return new_flow;
                }

                q.push_back((*next, new_flow));
            }
        }
    }

    0
}

fn calculate_max_flow(node_count: usize, source_node: usize, sink_node: usize, graph: &Vec<Vec<usize>>, capacity: &mut Vec<Vec<i32>>) -> Vec<bool> {
    let mut flow = 0;
    let mut parent: Vec<i32> = vec![0; node_count + 2];

    loop {
        let new_flow = bfs(source_node, sink_node, &mut parent, graph, capacity);
        if new_flow == 0 { break; }

        flow += new_flow;

        let mut curr = sink_node;
        while curr != source_node {
            let prev = parent[curr];
            capacity[prev as usize][curr] -= new_flow;
            capacity[curr][prev as usize] += new_flow;
            curr = prev as usize;
        }
    }

    println!("Calculated flow is {}", flow);
    capacity.into_iter().for_each(|row| println!("{:?}", row));

    // Find nodes reachable from the source after the flow is calculated
    let mut visited = vec![false; node_count + 2];
    let mut q = VecDeque::new();

    visited[source_node] = true;
    q.push_back(source_node);

    while let Some(node) = q.pop_front() {
        for &next in &graph[node] {
            if !visited[next] && capacity[node][next] > 0 {
                visited[next] = true;
                q.push_back(next);
            }
        }
    }

    visited
}

fn main() {
    let node_count: usize = 4;
    let (source_node, sink_node) = (0usize, node_count + 1);

    let mut graph: Vec<Vec<usize>> = vec![vec![]; node_count + 2];
    let mut capacity: Vec<Vec<i32>> = vec![vec![0; node_count + 2]; node_count + 2];

    graph[source_node].extend_from_slice(&[1, 4]);
    graph[1].extend_from_slice(&[2, 3]);
    graph[2].extend_from_slice(&[sink_node]);
    graph[3].extend_from_slice(&[2, sink_node]);
    graph[4].extend_from_slice(&[1, 3]);

    capacity[source_node][1] = 7;
    capacity[source_node][4] = 4;
    capacity[1][2] = 5;
    capacity[1][3] = 3;
    capacity[2][sink_node] = 8;
    capacity[3][2] = 3;
    capacity[3][sink_node] = 5;
    capacity[4][1] = 3;
    capacity[4][3] = 2;

    let visited_nodes = calculate_max_flow(node_count, source_node, sink_node, &graph, &mut capacity);

    for (index, node) in visited_nodes.iter().enumerate() {
        match *node {
            true => { println!("Node {} should be included", index); }
            false => { println!("Node {} should not be included", index); }
        }
    }
}