fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let data = input
        .trim()
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(' ').unwrap();
            (l, r.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let max = data.iter().map(|(l, b)| to_base10(l, *b)).max().unwrap();
    println!("p1: {}", max);

    // p2
    let sum = data.iter().map(|(l, b)| to_base10(l, *b)).sum::<u64>();
    println!("p2: {}", to_base68_string(sum));

    // p3
    println!("p3: {}", smallest_base_with_4_digits(sum));
}

fn smallest_base_with_4_digits(n: u64) -> u64 {
    let mut base = 0u64;
    while n >= base.pow(4) {
        base += 1;
    }
    base
}

fn to_base10(value_str: &str, base: u64) -> u64 {
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#";
    let mut result = 0;
    let mut power = 0;

    for c in value_str.chars().rev() {
        let index = alphabet.find(c).unwrap();
        result += index as u64 * base.pow(power);
        power += 1;
    }
    result
}

fn to_base68_string(mut x: u64) -> String {
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^";
    let mut result = vec![];
    let radix = 68;

    loop {
        let d = x % radix;
        x = x / radix;
        result.push(alphabet.chars().nth(d as usize).unwrap());

        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
