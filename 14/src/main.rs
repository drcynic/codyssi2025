use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let mats = input
        .trim()
        .lines()
        .map(|l| {
            let splits = l.split_ascii_whitespace().collect::<Vec<_>>();
            let name = splits[1];
            let qual = splits[5][..splits[5].len() - 1].parse::<usize>().unwrap();
            let cost = splits[8][..splits[8].len() - 1].parse::<usize>().unwrap();
            let um = splits[12].parse::<usize>().unwrap();

            (name, qual, cost, um)
        })
        .collect::<Vec<_>>();

    let p1 = mats
        .iter()
        .sorted_by(|a, b| if a.1 == b.1 { a.2.cmp(&b.2) } else { a.1.cmp(&b.1) })
        .rev()
        .take(5)
        .map(|(_, _, _, um)| um)
        .sum::<usize>();
    println!("Part 1: {}", p1);

    let mut best = (0, 0);
    knapsack(&mats, 0, (0, 0, 0), &mut best, &mut HashMap::new(), 30);
    println!("p2: {:?} -> {}", best, best.0 * best.1);

    let mut best = (0, 0);
    knapsack(&mats, 0, (0, 0, 0), &mut best, &mut HashMap::new(), 300);
    println!("p3: {:?} -> {}", best, best.0 * best.1);
}

fn knapsack<'a>(
    mats: &'a Vec<(&'a str, usize, usize, usize)>,
    idx: usize,
    cur: (usize, usize, usize),
    best: &mut (usize, usize),
    cache: &mut HashMap<(usize, usize), usize>,
    max: usize,
) {
    if cur.1 > max {
        return;
    }

    if cur.0 > best.0 || (cur.0 == best.0 && cur.2 < best.1) {
        best.0 = cur.0;
        best.1 = cur.2;
    }

    let k = (idx, cur.1); // idx + cost
    if let Some(qual) = cache.get(&k)
        && *qual > cur.0
    {
        return;
    }
    cache.insert(k, cur.0);

    if idx >= mats.len() - 1 {
        return;
    }

    knapsack(mats, idx + 1, (cur.0, cur.1, cur.2), best, cache, max); // ignore current
    knapsack(
        mats,
        idx + 1,
        (cur.0 + mats[idx + 1].1, cur.1 + mats[idx + 1].2, cur.2 + mats[idx + 1].3),
        best,
        cache,
        max,
    ); // add current
}
