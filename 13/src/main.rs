use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use priority_queue::PriorityQueue;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let mut all_vertices = HashSet::new();
    let edges_by_vertex: HashMap<&str, Vec<(&str, usize)>> = input.trim().lines().fold(HashMap::new(), |mut acc, l| {
        let (start, lr) = l.split_once(" -> ").unwrap();
        all_vertices.insert(start);
        let entry = acc.entry(start).or_insert(Vec::new());
        let (end, weight) = lr.split_once(" | ").unwrap();
        all_vertices.insert(end);
        let weight = weight.parse::<usize>().unwrap();
        entry.push((end, weight));
        acc
    });

    let p1: usize = all_vertices
        .iter()
        .map(|k| dykstra(&edges_by_vertex, "STT", k, false))
        .sorted()
        .rev()
        .take(3)
        .product();
    println!("p1: {}", p1);

    let p2: usize = all_vertices
        .iter()
        .map(|k| dykstra(&edges_by_vertex, "STT", k, true))
        .sorted()
        .rev()
        .take(3)
        .product();
    println!("p2: {}", p2);

    let p3 = all_vertices
        .iter()
        .map(|v| {
            let mut max_path = Vec::new();
            dfs(&edges_by_vertex, v, v, &mut vec![(v, 0)], &mut max_path);
            (max_path.len(), max_path.iter().map(|(_, w)| w).sum::<usize>())
        })
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .rev()
        .take(1)
        .map(|(_, c)| c)
        .sum::<usize>();
    println!("p3: {}", p3);
}

fn dykstra(nodes: &HashMap<&str, Vec<(&str, usize)>>, start: &str, target: &str, use_weight: bool) -> usize {
    let mut visited: HashMap<&str, usize> = HashMap::new();
    let mut pq = PriorityQueue::new();
    pq.push(start, Reverse(0));
    while let Some((current, cost)) = pq.pop() {
        if let Some(vc) = visited.get(&current) {
            if *vc <= cost.0 {
                continue;
            }
        }
        visited.insert(current, cost.0);

        if current == target {
            return cost.0;
        }

        if let Some(edges) = nodes.get(current) {
            for (end, weight) in edges {
                let weight = if use_weight { weight } else { &1 };
                let cost = Reverse(cost.0 + weight);
                if let Some(e) = pq.get(end) {
                    if *e.1 < cost {
                        pq.change_priority(end, cost);
                    }
                } else {
                    pq.push(end, cost);
                }
            }
        }
    }
    0
}

fn dfs<'a>(
    edges_by_node: &HashMap<&'a str, Vec<(&'a str, usize)>>,
    current: &'a str,
    target: &str,
    path: &mut Vec<(&'a str, usize)>,
    max_path: &mut Vec<(&'a str, usize)>,
) {
    if path.len() > 1 && current == path[0].0 {
        if path.len() > max_path.len() {
            *max_path = path.clone();
        }
        return;
    }

    if let Some(edges) = edges_by_node.get(current) {
        for (end, weight) in edges {
            if let Some(_) = path[1..].iter().find(|&(node, _)| node == end) {
                continue; // already visited
            }
            path.push((end, *weight));
            dfs(edges_by_node, end, target, path, max_path);
            path.pop();
        }
    }
}
