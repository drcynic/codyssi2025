use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let (func_str, quant_str) = input.trim().split_once("\n\n").unwrap();
    let values = func_str.split("\n").map(|s| get_val(s)).collect::<Vec<_>>();
    let (add, mul, pow) = (values[0], values[1], values[2]);
    let pricing = |median: i64| median.pow(pow as u32) * mul + add;
    let quants = quant_str
        .split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .sorted()
        .collect::<Vec<_>>();
    let median = quants[quants.len() / 2];
    let res = pricing(median);
    println!("Part1: {}", res);

    let sum = quants.iter().filter(|v| *v & 1 == 0).sum::<i64>();
    println!("Part2: {}", pricing(sum));

    let s = quants
        .iter()
        .map(|v| pricing(*v))
        .filter(|v| *v < 15000000000000i64)
        .sorted()
        .collect::<Vec<_>>();
    let v = s.last().unwrap();
    let room = f64::powf(((v - add) / mul) as f64, 1.0 / pow as f64) as i64 + 1;
    println!("Part3: {}", room);
}

fn get_val(s: &str) -> i64 {
    s.split_whitespace().last().unwrap().parse().unwrap()
}
