use std::{cmp::min, collections::HashSet};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();

    let p1 = input
        .trim()
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| (c - b'A' + 1) as i64).sum::<i64>())
        .sum::<i64>();
    println!("p1: {p1}");

    let p2 = input
        .trim()
        .lines()
        .map(|l| {
            let n = l.len() / 10;
            let s = format!("{}{}{}", &l[..n], l.len() - 2 * n, &l[l.len() - n..]);
            count(&s)
        })
        .sum::<i64>();
    println!("p2: {p2}");

    let p3: i64 = input
        .trim()
        .lines()
        .map(|l| {
            let chars = l.as_bytes();
            let mut r = String::new();
            let mut cc = chars[0];
            let mut num = 1;
            for c in &chars[1..] {
                if *c != cc {
                    r.push_str(&num.to_string());
                    r.push(cc as char);
                    cc = *c;
                    num = 1;
                } else {
                    num += 1;
                }
            }
            r.push_str(&num.to_string());
            r.push(cc as char);
            count(&r)
        })
        .sum();
    println!("p3: {p3}");
}

fn count(l: &str) -> i64 {
    l.as_bytes()
        .iter()
        .map(|c| match c {
            b'A'..=b'Z' => (c - b'A' + 1) as i64,
            b'0'..=b'9' => (c - b'0') as i64,
            _ => unreachable!(),
        })
        .sum::<i64>()
}
