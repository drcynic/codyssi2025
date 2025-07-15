fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let num_valid = input.trim().as_bytes().iter().filter(|c| c.is_ascii_alphabetic()).count();
    println!("p1: {}", num_valid);

    let sum = input
        .trim()
        .as_bytes()
        .iter()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| match c {
            b'a'..=b'z' => (c - b'a' + 1) as i64,
            b'A'..=b'Z' => (c - b'A' + 27) as i64,
            _ => unreachable!(),
        })
        .sum::<i64>();
    println!("p2: {}", sum);

    let log = input.trim().as_bytes().iter().map(|c| to_val(c)).collect::<Vec<_>>();
    let mut values = Vec::new();
    let mut idx = 0;
    while idx < log.len() {
        let v = if let Some(v) = log[idx] {
            v
        } else {
            (values[idx - 1] * 2 - 5 + 5200) % 52
        };
        values.push(v);
        idx += 1;
    }
    let sum = values.iter().sum::<i64>();
    println!("p3: {}", sum);
}

fn to_val(c: &u8) -> Option<i64> {
    match c {
        b'a'..=b'z' => Some((c - b'a' + 1) as i64),
        b'A'..=b'Z' => Some((c - b'A' + 27) as i64),
        _ => None,
    }
}
