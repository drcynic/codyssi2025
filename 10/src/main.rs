use std::{cmp::Reverse, collections::HashMap};

use priority_queue::PriorityQueue;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let input_blocks = input.trim().split("\n\n").collect::<Vec<_>>();
    let grid = input_blocks[0]
        .lines()
        .map(|line| line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("p1: {}", get_min_col_row(&grid));
    println!("p2: {}", bfs_down_right(&grid, (14, 14)));
    println!("p3: {}", bfs_down_right(&grid, (grid.len() - 1, grid[0].len() - 1)));
}

fn bfs_down_right(grid: &Vec<Vec<i64>>, target_pos: (usize, usize)) -> i64 {
    let start_pos = (0, 0);
    let mut visited: HashMap<(usize, usize), i64> = HashMap::new();
    let mut pq = PriorityQueue::new();
    pq.push(start_pos, Reverse(grid[start_pos.1][start_pos.0]));
    while let Some((current, cost)) = pq.pop() {
        if let Some(vc) = visited.get(&current) {
            if *vc <= cost.0 {
                continue;
            }
        }
        visited.insert(current, cost.0);

        let (x, y) = current;
        if current == target_pos {
            return cost.0;
        }

        if x < target_pos.0 {
            update_or_insert(grid, &mut pq, (x + 1, y), cost);
        }
        if y < target_pos.1 {
            update_or_insert(grid, &mut pq, (x, y + 1), cost);
        }
    }
    0
}

fn update_or_insert(grid: &Vec<Vec<i64>>, pq: &mut PriorityQueue<(usize, usize), Reverse<i64>>, pos: (usize, usize), cost: Reverse<i64>) {
    let cost = Reverse(cost.0 + grid[pos.1][pos.0]);
    if let Some(e) = pq.get(&pos) {
        if *e.1 < cost {
            pq.change_priority(&pos, cost);
        }
    } else {
        pq.push(pos, cost);
    }
}

fn get_min_col_row(grid: &Vec<Vec<i64>>) -> i64 {
    let row_sums_min = grid.iter().map(|row| row.iter().sum::<i64>()).min().unwrap();
    let col_sums_min = (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).sum::<i64>())
        .min()
        .unwrap();
    std::cmp::min(row_sums_min, col_sums_min)
}
