use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Face(usize),
    Row(usize, usize),
    Col(usize, usize),
}

#[derive(Debug, Clone, Copy)]
struct Face<const N: usize> {
    grid: [[usize; N]; N],
    cons: [usize; 4], // left, up, right, down
    rot: usize,
}

impl<const N: usize> Face<N> {
    fn new(init_val: usize, cons: [usize; 4]) -> Self {
        Face {
            grid: [[init_val; N]; N],
            cons,
            rot: 0,
        }
    }

    fn rot_right(&mut self, n: usize) {
        for _ in 0..n {
            let mut new_grid = [[0; N]; N];
            for i in 0..N {
                for j in 0..N {
                    new_grid[j][N - 1 - i] = self.grid[i][j];
                }
            }
            self.grid = new_grid;
            self.cons.rotate_right(1);
            println!("rotated cons: {:?}", self.cons);
        }
        self.rot += n;
        println!("rotated right by {} to {}", n, self.rot);
    }

    fn rot_left(&mut self, n: usize) {
        for _ in 0..n {
            let mut new_grid = [[0; N]; N];
            for i in 0..N {
                for j in 0..N {
                    new_grid[N - 1 - j][i] = self.grid[i][j];
                }
            }
            self.grid = new_grid;
            self.cons.rotate_left(1);
        }
        self.rot -= n;
    }
}

#[derive(Debug)]
struct Cube<const N: usize> {
    faces: [Face<N>; 6],
    cur_idx: usize,
    cur_face: Face<N>,
}

impl<const N: usize> Cube<N> {
    fn new_p1() -> Self {
        Cube {
            faces: [
                Face::<N>::new(0, [4, 3, 5, 1]),
                Face::<N>::new(0, [4, 0, 5, 2]),
                Face::<N>::new(0, [4, 1, 5, 3]),
                Face::<N>::new(0, [4, 2, 5, 0]),
                Face::<N>::new(0, [3, 0, 1, 2]),
                Face::<N>::new(0, [1, 0, 3, 2]),
            ],
            cur_idx: 0,
            cur_face: Face::<N>::new(0, [4, 3, 5, 1]),
        }
    }

    fn new_p2() -> Self {
        Cube {
            faces: [
                Face::<N>::new(1, [4, 3, 5, 1]),
                Face::<N>::new(1, [4, 0, 5, 2]),
                Face::<N>::new(1, [4, 1, 5, 3]),
                Face::<N>::new(1, [4, 2, 5, 0]),
                Face::<N>::new(1, [3, 0, 1, 2]),
                Face::<N>::new(1, [1, 0, 3, 2]),
            ],
            cur_idx: 0,
            cur_face: Face::<N>::new(1, [4, 3, 5, 1]),
        }
    }

    fn apply_instr_p1(&mut self, instruction: &Instruction) {
        println!("Applying instruction: {:?}", instruction);
        let cf = &mut self.cur_face;
        match instruction {
            Instruction::Face(val) => cf.grid[0][0] += N * N * val,
            Instruction::Row(_, val) => cf.grid[0][0] += N * val,
            Instruction::Col(_, val) => cf.grid[0][0] += N * val,
        }
        self.correct();
    }

    fn apply_instr_p2(&mut self, instruction: &Instruction) {
        println!("Applying instruction: {:?}", instruction);
        let cf = &mut self.cur_face;
        match instruction {
            Instruction::Face(val) => cf.grid.iter_mut().for_each(|row| row.iter_mut().for_each(|u| *u += val)),
            Instruction::Row(row, val) => cf.grid[*row].iter_mut().for_each(|u| *u += val),
            Instruction::Col(col, val) => cf.grid.iter_mut().for_each(|row| row[*col] += val),
        }
        self.correct();
        self.copy_back();
    }

    fn correct(&mut self) {
        let cf = &mut self.cur_face;
        cf.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|u| {
                while *u > 100 {
                    *u -= 100;
                }
            })
        });
    }

    fn copy_back(&mut self) {
        self.faces[self.cur_idx] = self.cur_face;
        self.faces[self.cur_idx].rot_left(self.cur_face.rot);
    }

    fn rotate(&mut self, dir: u8) {
        // prepare new current face
        let f = self.cur_face;
        let mut nf = self.faces[self.cur_face.cons[0]];
        match dir {
            b'L' => {
                println!("Left");
                match self.cur_idx {
                    0 => match f.cons[0] {
                        4 | 1 | 5 => nf.rot_right(1),
                        3 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    1 => match f.cons[0] {
                        4 => nf.rot_right(0),
                        2 => nf.rot_right(1),
                        5 => nf.rot_right(2),
                        0 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    2 => match f.cons[0] {
                        4 | 5 | 1 => nf.rot_right(3),
                        3 => nf.rot_right(1),
                        _ => unreachable!(),
                    },
                    3 => match f.cons[0] {
                        4 => nf.rot_right(2),
                        0 => nf.rot_right(1),
                        5 => nf.rot_right(0),
                        2 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    4 => match f.cons[0] {
                        3 | 0 | 2 | 1 => nf.rot_right(2),
                        _ => unreachable!(),
                    },
                    5 => match f.cons[0] {
                        1 | 2 | 3 | 0 => nf.rot_right(0),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
                self.cur_idx = self.cur_face.cons[0];
            }
            b'R' => {
                println!("Right");
                nf = self.faces[self.cur_face.cons[2]];
                match self.cur_idx {
                    0 => match f.cons[2] {
                        5 | 4 | 1 => nf.rot_right(3),
                        3 => nf.rot_right(1),
                        _ => unreachable!(),
                    },
                    1 => match f.cons[2] {
                        5 => nf.rot_right(0),
                        0 => nf.rot_right(1),
                        4 => nf.rot_right(2),
                        2 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    2 => match f.cons[2] {
                        5 | 1 | 4 => nf.rot_right(1),
                        3 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    3 => match f.cons[2] {
                        5 => nf.rot_right(2),
                        2 => nf.rot_right(1),
                        4 => nf.rot_right(0),
                        0 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    4 => match f.cons[2] {
                        1 | 0 | 3 | 2 => nf.rot_right(0),
                        _ => unreachable!(),
                    },
                    5 => match f.cons[2] {
                        3 | 0 | 1 | 2 => nf.rot_right(2),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
                self.cur_idx = self.cur_face.cons[2];
            }
            b'U' => {
                println!("Up");
                nf = self.faces[self.cur_face.cons[1]];
                match self.cur_idx {
                    0 => match f.cons[1] {
                        3 => nf.rot_right(0),
                        4 | 1 | 5 => nf.rot_right(2),
                        _ => unreachable!(),
                    },
                    1 => match f.cons[1] {
                        0 => nf.rot_right(0),
                        4 => nf.rot_right(1),
                        2 => nf.rot_right(2),
                        5 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    2 => match f.cons[1] {
                        1 | 4 | 5 => nf.rot_right(0),
                        3 => nf.rot_right(2),
                        _ => unreachable!(),
                    },
                    3 => match f.cons[1] {
                        2 => nf.rot_right(0),
                        4 => nf.rot_right(3),
                        0 => nf.rot_right(2),
                        5 => nf.rot_right(1),
                        _ => unreachable!(),
                    },
                    4 => match f.cons[1] {
                        0 | 3 | 2 | 1 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    5 => match f.cons[1] {
                        0 | 3 | 2 | 1 => nf.rot_right(1),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
                self.cur_idx = self.cur_face.cons[1];
            }
            b'D' => {
                println!("Down");
                nf = self.faces[self.cur_face.cons[3]];
                match self.cur_idx {
                    0 => match f.cons[3] {
                        1 | 5 | 4 => nf.rot_right(0),
                        3 => nf.rot_right(2),
                        _ => unreachable!(),
                    },
                    1 => match f.cons[3] {
                        2 => nf.rot_right(0),
                        5 => nf.rot_right(1),
                        0 => nf.rot_right(2),
                        4 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    2 => match f.cons[3] {
                        3 => nf.rot_right(0),
                        5 | 1 | 4 => nf.rot_right(2),
                        _ => unreachable!(),
                    },
                    3 => match f.cons[3] {
                        0 => nf.rot_right(0),
                        5 => nf.rot_right(3),
                        2 => nf.rot_right(2),
                        4 => nf.rot_right(1),
                        _ => unreachable!(),
                    },
                    4 => match f.cons[3] {
                        2 | 1 | 0 | 3 => nf.rot_right(1),
                        _ => unreachable!(),
                    },
                    5 => match f.cons[3] {
                        2 | 3 | 0 | 1 => nf.rot_right(3),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
                self.cur_idx = self.cur_face.cons[3];
            }
            _ => unreachable!(),
        }
        // set new current
        self.cur_face = nf;
        println!("cur idx: {} , cur rot: {}", self.cur_idx, self.cur_face.rot);
        // self.print_faces();
    }

    fn p1(&self) {
        let p1 = self
            .faces
            .iter()
            .map(|&face| face.grid[0][0])
            .sorted()
            .rev()
            .take(2)
            .product::<usize>();
        println!("p1: {}", p1);
    }

    fn dominant_sum(&self, grid: &[[usize; N]; N]) -> usize {
        let row_max = grid.iter().map(|r| r.iter().sum::<usize>()).max().unwrap();
        let col_max = (0..N).map(|c| (0..N).map(|r| grid[r][c]).sum::<usize>()).max().unwrap();
        row_max.max(col_max)
    }

    fn p2(&self) {
        let p2 = self
            .faces
            .iter()
            .map(|&face| self.dominant_sum(&face.grid) as i128)
            .product::<i128>();
        println!("p2: {}", p2);
    }
}

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let (instr, twists) = input.split_once("\n\n").unwrap();
    let instr = instr
        .trim()
        .lines()
        .map(|l| {
            let (cmd, val) = l.split_once(" - VALUE ").unwrap();
            let cmd = match &cmd[..4] {
                "FACE" => Instruction::Face(val.parse().unwrap()),
                "ROW " => Instruction::Row(cmd[4..].parse::<usize>().unwrap() - 1, val.parse().unwrap()),
                "COL " => Instruction::Col(cmd[4..].parse::<usize>().unwrap() - 1, val.parse().unwrap()),
                _ => unreachable!(),
            };
            cmd
        })
        .collect::<Vec<_>>();

    let mut cube: Cube<3> = Cube::new_p1();
    // cube.print_faces();
    // twists
    //     .trim()
    //     .as_bytes()
    //     .iter()
    //     .zip(&instr[..instr.len() - 1])
    //     .for_each(|(twist, i)| {
    //         cube.apply_instr_p1(i);
    //         cube.rotate(*twist);
    //     });
    // cube.apply_instr_p1(instr.last().unwrap());
    // cube.p1();

    let mut cube: Cube<80> = Cube::new_p2();
    println!("cube: {:?}", &cube);
    twists
        .trim()
        .as_bytes()
        .iter()
        .zip(&instr[..instr.len() - 1])
        .for_each(|(twist, i)| {
            cube.apply_instr_p2(i);
            // println!("cube: {:?}", &cube);
            cube.rotate(*twist);
        });
    cube.apply_instr_p2(instr.last().unwrap());
    cube.faces[cube.cur_idx] = cube.cur_face;
    // println!("cube: {:?}", &cube);
    cube.p2();
}
