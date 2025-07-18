use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();

    let p1 = input.as_bytes().iter().filter(|c| c.is_ascii_alphabetic()).count();
    println!("p1: {}", p1);

    // p2
    let re = Regex::new(r"(([a-z]|-)\d|\d([a-z]|-))").unwrap();
    let num = reduce_with_regex(&input, re);
    println!("p2: {}", num);

    // p3
    let re = Regex::new(r"([a-z]\d|\d[a-z])").unwrap();
    let num = reduce_with_regex(&input, re);
    println!("p3: {}", num);
}

fn reduce_with_regex(input: &str, re: Regex) -> usize {
    input
        .lines()
        .map(|l| {
            let mut reduced = l.to_string();
            while let Some(m) = re.find(&reduced) {
                reduced = format!("{}{}", &reduced[..m.start()], &reduced[m.end()..]);
            }
            reduced.len()
        })
        .sum()
}
