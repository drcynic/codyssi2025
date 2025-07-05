use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let lines = input.trim().lines().collect::<Vec<_>>();
    let signs = lines.last().unwrap().chars().collect::<Vec<_>>();
    let values = lines[..lines.len() - 1]
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let p1 = sum(&signs, &values);
    println!("Part1: {}", p1);

    let signs = lines.last().unwrap().chars().rev().collect::<Vec<_>>();
    let p2 = sum(&signs, &values);
    println!("Part2: {}", p2);

    let values = lines[..lines.len() - 1]
        .iter()
        .tuples()
        .map(|(l, r)| l.parse::<i32>().unwrap() * 10 + r.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let p3 = sum(&signs, &values);
    println!("Part3: {}", p3);
}

fn sum(signs: &[char], values: &[i32]) -> i32 {
    values
        .iter()
        .enumerate()
        .map(|(idx, v)| {
            if idx == 0 {
                *v
            } else if signs[idx - 1] == '-' {
                -*v
            } else {
                *v
            }
        })
        .sum::<i32>()
}
