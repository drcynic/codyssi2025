use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input1a.txt").unwrap();
    let split_input = input.split("\n\n").collect::<Vec<&str>>();
    let tracks_orig = split_input[0]
        .split("\n")
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let swaps = split_input[1]
        .split("\n")
        .map(|l| {
            let (l, r) = l.split_once('-').unwrap();
            (l.parse::<usize>().unwrap() - 1, r.parse::<usize>().unwrap() - 1)
        })
        .collect::<Vec<(usize, usize)>>();
    let test_idx = split_input[2].trim().parse::<usize>().unwrap();

    // p1
    let mut tracks = tracks_orig.clone();
    swaps.iter().for_each(|(l, r)| {
        tracks.swap(*l, *r);
    });
    println!("p1: {}", tracks[test_idx - 1]);

    // p2
    let mut tracks = tracks_orig.clone();
    swaps
        .iter()
        .circular_tuple_windows()
        .take(swaps.len())
        .for_each(|((x, y), (z, _))| {
            tracks.swap(*z, *y);
            tracks.swap(*x, *y);
        });
    println!("p2: {}", tracks[test_idx - 1]);

    // p3
    let mut tracks = tracks_orig.clone();
    swaps.iter().for_each(|(l, r)| {
        let max_start = std::cmp::max(*l, *r);
        let mut li = std::cmp::min(*l, *r);
        let mut ri = max_start;
        while li < max_start && ri < tracks.len() {
            tracks.swap(li, ri);
            li += 1;
            ri += 1;
        }
    });
    println!("p3: {}", tracks[test_idx - 1]);
}
