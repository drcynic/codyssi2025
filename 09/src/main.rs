use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let (names_str, trans_str) = input.trim().split_once("\n\n").unwrap();
    let initial_balances: HashMap<&str, i64> = names_str
        .lines()
        .map(|line| {
            let (name, amount) = line.split_once(" HAS ").unwrap();
            (name, amount.parse().unwrap())
        })
        .collect();
    let transactions = trans_str
        .lines()
        .map(|l| {
            let (from, to_and_amount) = l[5..].split_once(" TO ").unwrap();
            let (to, amount) = to_and_amount.split_once(" AMT ").unwrap();
            let amount = amount.parse::<i64>().unwrap();
            (from, to, amount)
        })
        .collect::<Vec<_>>();

    // p1
    let mut balances = initial_balances.clone();
    transactions.iter().for_each(|(from, to, amount)| {
        *balances.get_mut(from).unwrap() -= amount;
        *balances.get_mut(to).unwrap() += amount;
    });
    let sum = balances.values().sorted().rev().take(3).sum::<i64>();
    println!("p1: {}", sum);

    // p2
    let mut balances = initial_balances.clone();
    transactions.iter().for_each(|(from, to, amount)| {
        let amount = std::cmp::min(*balances.get(from).unwrap(), *amount);
        *balances.get_mut(from).unwrap() -= amount;
        *balances.get_mut(to).unwrap() += amount;
    });
    let sum = balances.values().sorted().rev().take(3).sum::<i64>();
    println!("p2: {}", sum);

    // p3
    let mut balances = initial_balances.clone();
    let mut debts: HashMap<&str, VecDeque<(&str, i64)>> = HashMap::new();
    transactions.iter().for_each(|(from, to, amount)| {
        let mut amount = *amount;
        let from_balance = *balances.get(from).unwrap();
        if amount > from_balance {
            let entry = debts.entry(from).or_default();
            entry.push_back((to, amount - from_balance));
            amount = from_balance;
        }
        *balances.get_mut(from).unwrap() -= amount;
        *balances.get_mut(to).unwrap() += amount;
        resolve_debts(&mut balances, &mut debts);
    });
    let sum = balances.values().sorted().rev().take(3).sum::<i64>();
    println!("p3: {}", sum);
}

fn resolve_debts<'a>(balances: &mut HashMap<&'a str, i64>, debts: &mut HashMap<&'a str, VecDeque<(&'a str, i64)>>) {
    for (k, v) in debts {
        let mut amount = *balances.get(k).unwrap();
        loop {
            if v.is_empty() {
                break;
            }
            if amount > v.front().unwrap().1 {
                let (debt_to, debt) = v.pop_front().unwrap();
                *balances.get_mut(debt_to).unwrap() += debt;
                *balances.get_mut(k).unwrap() -= debt;
                amount -= debt;
            } else {
                let (debt_to, debt) = v.front_mut().unwrap();
                *debt -= amount;
                *balances.get_mut(debt_to).unwrap() += amount;
                *balances.get_mut(k).unwrap() -= amount;
                break;
            }
        }
    }
}
