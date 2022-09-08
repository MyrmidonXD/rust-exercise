use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: usize,
    board: Vec<u8>
}

#[derive(Debug)]
struct Matrix<T> {
    row: usize,
    col: usize,
    data: Vec<T>
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

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match index {
            (y, _) if y >= self.row => { panic!("index out of bounds: the row size is {} but the row index is {}", self.row, y); },
            (_, x) if x >= self.col => { panic!("index out of bounds: the column size is {} but the column index is {}", self.col, x); }
            (y, x) => { return &mut self.data[y * self.col + x]; }
        }
    }
}

fn count_square(board: &Matrix<u8>, start: (usize, usize), end: (usize, usize)) -> (usize, usize) {
    let (y1, x1) = start;
    let (y2, x2) = end;

    let color = board[(y1, x1)];
    let mut uniform = true;

    'outer: for y in y1..y2 {
        for x in x1..x2 {
            if board[(y, x)] != color {
                uniform = false;
                break 'outer;
            }
        }
    }

    if uniform {
        return if color == 0 { (1, 0) } else { (0, 1) };
    }

    let (ym, xm) = ((y1+y2)/2, (x1+x2)/2);
    
    let result = IntoIterator::into_iter([
        count_square(board, (y1, x1), (ym, xm)),
        count_square(board, (y1, xm), (ym, x2)),
        count_square(board, (ym, x1), (y2, xm)),
        count_square(board, (ym, xm), (y2, x2)),
    ]).fold((0, 0), |(acc_a, acc_b), (a, b)| (acc_a+a, acc_b+b));

    result
}

fn solve(tc: TestCase) {
    let board = Matrix {
        row: tc.n,
        col: tc.n,
        data: tc.board,
    };
    
    let (num_white, num_blue) = count_square(&board, (0, 0), (tc.n, tc.n));
    println!("{num_white}");
    println!("{num_blue}");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().expect("IO error").parse().unwrap();
                                  
    let mut board: Vec<u8> = Vec::new();

    while let Some(line) = lines.next() {
        line.unwrap().split_whitespace().for_each(|c| board.push(c.parse().unwrap()));
    }

    solve(TestCase { n, board });
}