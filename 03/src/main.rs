use std::{cmp::min, collections::HashSet};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();

    let total = input
        .trim()
        .split_ascii_whitespace()
        .map(|r| {
            let (s, e) = r.split_once('-').unwrap();
            e.parse::<i64>().unwrap() - s.parse::<i64>().unwrap() + 1
        })
        .sum::<i64>();
    println!("p1: {total}");

    let boxes = input
        .trim()
        .split('\n')
        .map(|l| {
            let mut boxes = HashSet::new();
            add(l, &mut boxes);
            boxes.len() as i64
        })
        .sum::<i64>();
    println!("p2: {boxes}");

    let boxes = input
        .trim()
        .split('\n')
        .tuple_windows()
        .map(|(l1, l2)| {
            let mut boxes = HashSet::new();
            add(l1, &mut boxes);
            add(l2, &mut boxes);
            boxes.len() as i64
        })
        .max()
        .unwrap();
    println!("p3: {boxes}");
}

fn add(l: &str, boxes: &mut HashSet<i64>) {
    let (r1, r2) = l.split_once(' ').unwrap();
    let (s1, e1) = r1.split_once('-').unwrap();
    let (s1, e1) = (s1.parse::<i64>().unwrap(), e1.parse::<i64>().unwrap());
    let (s2, e2) = r2.split_once('-').unwrap();
    let (s2, e2) = (s2.parse::<i64>().unwrap(), e2.parse::<i64>().unwrap());
    for b in s1..=e1 {
        boxes.insert(b);
    }
    for b in s2..=e2 {
        boxes.insert(b);
    }
}
