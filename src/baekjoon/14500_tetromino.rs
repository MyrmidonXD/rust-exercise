use std::io::{self, BufRead};
use std::ops::{Index};
use std::convert::{TryInto};

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: usize,
    m: usize,
    cells: Vec<usize>
}

#[derive(Debug)]
struct Matrix<T> {
    row: usize,
    col: usize,
    data: Vec<T>
}

#[derive(Debug)]
struct Tetromino {
    row: usize,
    col: usize,
    cells: [(usize, usize); 4]
}

impl Tetromino {
    fn new(cells: [(usize, usize); 4]) -> Self {
        let mut y_max = 0;
        let mut x_max = 0;

        for (y, x) in cells {
            if y > y_max { y_max = y }
            if x > x_max { x_max = x }
        }

        Self { row: y_max + 1, col: x_max + 1, cells }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match index {
            (y, _) if y >= self.row => { panic!("index out of bounds: the row size is {} but the row index is {}", self.row, y); },
            (_, x) if x >= self.col => { panic!("index out of bounds: the column size is {} but the column index is {}", self.col, x); }
            (y, x) => { return &self.data[y * self.col + x]; }
        }
    }
}

fn get_sum(board: &Matrix<usize>, block: &Tetromino, pivot: (usize, usize)) -> Option<usize> {
    let (y, x) = pivot;

    if y + block.row > board.row || x + block.col > board.col {
        return None;
    }

    let mut sum = 0;
    for (y_ofs, x_ofs) in block.cells {
        sum += board[(y+y_ofs, x+x_ofs)];
    }

    Some(sum)
}

fn solve(tc: TestCase) {
    let board = Matrix {
        row: tc.n,
        col: tc.m,
        data: tc.cells,
    };
    let blocks = [
        Tetromino::new([(0, 0), (0, 1), (0, 2), (0, 3)]),
        Tetromino::new([(0, 0), (1, 0), (2, 0), (3, 0)]),
        Tetromino::new([(0, 0), (0, 1), (1, 0), (1, 1)]),
        Tetromino::new([(0, 0), (1, 0), (2, 0), (2, 1)]),
        Tetromino::new([(0, 0), (0, 1), (0, 2), (1, 0)]),
        Tetromino::new([(0, 0), (0, 1), (1, 1), (2, 1)]),
        Tetromino::new([(0, 2), (1, 0), (1, 1), (1, 2)]),
        Tetromino::new([(0, 1), (1, 1), (2, 0), (2, 1)]),
        Tetromino::new([(0, 0), (0, 1), (0, 2), (1, 2)]),
        Tetromino::new([(0, 0), (0, 1), (1, 0), (2, 0)]),
        Tetromino::new([(0, 0), (1, 0), (1, 1), (1, 2)]),
        Tetromino::new([(0, 0), (1, 0), (1, 1), (2, 1)]),
        Tetromino::new([(0, 1), (0, 2), (1, 0), (1, 1)]),
        Tetromino::new([(0, 1), (1, 0), (1, 1), (2, 0)]),
        Tetromino::new([(0, 0), (0, 1), (1, 1), (1, 2)]),
        Tetromino::new([(0, 0), (0, 1), (0, 2), (1, 1)]),
        Tetromino::new([(0, 1), (1, 0), (1, 1), (2, 1)]),
        Tetromino::new([(0, 1), (1, 0), (1, 1), (1, 2)]),
        Tetromino::new([(0, 0), (1, 0), (1, 1), (2, 0)]),
    ];

    let mut max: usize = 0;

    for y in 0..board.row {
        for x in 0..board.col {
            for block in &blocks {
                match get_sum(&board, &block, (y, x)) {
                    Some(n) if n > max => { max = n },
                    _ => continue
                }
            }
        }
    }

    println!("{}", max);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let [n, m]: [usize; 2] = lines.next().unwrap().expect("IO error").split_whitespace()
                                    .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                                    .try_into().unwrap();

    let mut cells = Vec::new();
    for _ in 0..n {
        lines.next().unwrap().expect("IO error").split_whitespace()
            .for_each(|s| cells.push(s.parse::<usize>().unwrap()));
    }
    
    solve(TestCase { n, m, cells });
}