use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();

    let mut positions = input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l[1..l.len() - 1].split_once(", ").unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    let closest = positions.iter().map(|(x, y)| x.abs() + y.abs()).min().unwrap();
    let furthest = positions.iter().map(|(x, y)| x.abs() + y.abs()).max().unwrap();
    println!("p1: {:?}", furthest - closest);

    let closed_idx = positions
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(idx, _)| idx)
        .unwrap();
    let mut closest_pos = positions.remove(closed_idx);
    let closest_p2 = positions.iter().map(|p| manhattan_dist(&closest_pos, p)).min().unwrap();
    println!("p2: {closest_p2}");

    let mut sum = closest + closest_p2;
    while positions.len() > 1 {
        let closed_idx = positions
            .iter()
            .map(|p| manhattan_dist(&closest_pos, p))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap();
        closest_pos = positions.remove(closed_idx);
        let closest = positions.iter().map(|p| manhattan_dist(&closest_pos, p)).min().unwrap();
        sum += closest;
    }
    println!("p3: {sum}");
}

fn manhattan_dist(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}
