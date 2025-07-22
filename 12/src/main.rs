use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let input_blocks = input.trim().split("\n\n").collect::<Vec<_>>();
    let initial_grid = input_blocks[0]
        .lines()
        .map(|line| line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    print_grid(&initial_grid);
    println!();

    // p1
    let mut grid = initial_grid.clone();
    input_blocks[1].lines().for_each(|line| {
        apply_instr(line, &mut grid);
    });
    // print_grid(&grid);
    println!("p1: {}", get_max_col_row(grid));

    // p2
    let mut grid = initial_grid.clone();
    let mut instr = input_blocks[1].lines().collect::<VecDeque<_>>();
    let mut cur_action: &str = "";
    input_blocks[2].lines().for_each(|line| match line {
        "TAKE" => cur_action = instr.pop_front().unwrap(),
        "CYCLE" => instr.push_back(cur_action),
        "ACT" => apply_instr(cur_action, &mut grid),
        _ => unreachable!(),
    });
    println!("p2: {}", get_max_col_row(grid));

    // p3
    let mut grid = initial_grid.clone();
    let mut instr = input_blocks[1].lines().collect::<VecDeque<_>>();
    let mut cur_action: &str = "";
    for line in input_blocks[2].lines().cycle() {
        match line {
            "TAKE" => {
                if instr.is_empty() {
                    break;
                }
                cur_action = instr.pop_front().unwrap();
            }
            "CYCLE" => instr.push_back(cur_action),
            "ACT" => apply_instr(cur_action, &mut grid),
            _ => unreachable!(),
        }
    }
    println!("p3: {}", get_max_col_row(grid));
}

fn get_max_col_row(grid: Vec<Vec<i64>>) -> i64 {
    let row_sums_max = grid.iter().map(|row| row.iter().sum::<i64>()).max().unwrap();
    let col_sums_max = (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).sum::<i64>())
        .max()
        .unwrap();
    std::cmp::max(row_sums_max, col_sums_max)
}

fn apply_instr(l: &str, grid: &mut Vec<Vec<i64>>) {
    println!("Applying instruction: {}", l);
    let (cmd, param) = l.split_once(" ").unwrap();
    let param = param.split(" ").collect::<Vec<_>>();
    match cmd {
        "ADD" => {
            let amount = param[0].parse::<i64>().unwrap();
            match param[1] {
                "ALL" => {
                    grid.iter_mut().for_each(|row| row.iter_mut().for_each(|val| *val += amount));
                }
                "COL" => {
                    let idx = param[2].parse::<usize>().unwrap() - 1;
                    grid.iter_mut().for_each(|row| row[idx] += amount);
                }
                "ROW" => {
                    let idx = param[2].parse::<usize>().unwrap() - 1;
                    grid[idx].iter_mut().for_each(|val| *val += amount);
                }
                _ => unreachable!(),
            }
        }
        "SUB" => {
            let amount = param[0].parse::<i64>().unwrap();
            match param[1] {
                "ALL" => {
                    grid.iter_mut().for_each(|row| row.iter_mut().for_each(|val| *val -= amount));
                }
                "COL" => {
                    let idx = param[2].parse::<usize>().unwrap() - 1;
                    grid.iter_mut().for_each(|row| row[idx] -= amount);
                }
                "ROW" => {
                    let idx = param[2].parse::<usize>().unwrap() - 1;
                    grid[idx].iter_mut().for_each(|val| *val -= amount);
                }
                _ => unreachable!(),
            }
        }
        "MULTIPLY" => {
            let amount = param[0].parse::<i64>().unwrap();
            match param[1] {
                "ALL" => {
                    grid.iter_mut().for_each(|row| row.iter_mut().for_each(|val| *val *= amount));
                }
                "COL" => {
                    let idx = param[2].parse::<usize>().unwrap() - 1;
                    grid.iter_mut().for_each(|row| row[idx] *= amount);
                }
                "ROW" => {
                    let idx = param[2].parse::<usize>().unwrap() - 1;
                    grid[idx].iter_mut().for_each(|val| *val *= amount);
                }
                _ => unreachable!(),
            }
        }
        "SHIFT" => {
            let idx = param[1].parse::<usize>().unwrap() - 1;
            let shift = param[3].parse::<usize>().unwrap();
            match param[0] {
                "COL" => {
                    let mut col = grid.iter().map(|row| row[idx]).collect::<Vec<_>>();
                    col.rotate_right(shift);
                    grid.iter_mut().zip(col).for_each(|(row, val)| row[idx] = val);
                }
                "ROW" => {
                    grid[idx].rotate_right(shift);
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    check_grid_value_range(grid);
}

fn check_grid_value_range(grid: &mut Vec<Vec<i64>>) {
    grid.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|val| {
            while *val < 0 {
                *val += 1073741824;
            }
            while *val > 1073741823 {
                *val -= 1073741824;
            }
        })
    });
}

fn print_grid(grid: &Vec<Vec<i64>>) {
    for row in grid {
        println!("{:?}", row);
    }
}
