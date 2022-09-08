use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: usize,
    board: Vec<i8>
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

fn count_square(board: &Matrix<i8>, start: (usize, usize), end: (usize, usize)) -> (usize, usize, usize) {
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
        return if color == -1 { (1, 0, 0) } else if color == 0 { (0, 1, 0) } else { (0, 0, 1) };
    }

    let (y_d, x_d) = ((y2-y1)/3, (x2-x1)/3);
    
    let result = IntoIterator::into_iter([
        count_square(board, (y1, x1), (y1 + y_d, x1 + x_d)),
        count_square(board, (y1, x1 + x_d), (y1 + y_d, x1 + 2 * x_d)),
        count_square(board, (y1, x1 + 2 * x_d), (y1 + y_d, x2)),
        count_square(board, (y1 + y_d, x1), (y1 + 2 * y_d, x1 + x_d)),
        count_square(board, (y1 + y_d, x1 + x_d), (y1 + 2 * y_d, x1 + 2 * x_d)),
        count_square(board, (y1 + y_d, x1 + 2 * x_d), (y1 + 2 * y_d, x2)),
        count_square(board, (y1 + 2 * y_d, x1), (y2, x1 + x_d)),
        count_square(board, (y1 + 2 * y_d, x1 + x_d), (y2, x1 + 2 * x_d)),
        count_square(board, (y1 + 2 * y_d, x1 + 2 * x_d), (y2, x2)),
    ]).fold((0, 0, 0), |(acc_a, acc_b, acc_c), (a, b, c)| (acc_a+a, acc_b+b, acc_c+c));

    result
}

fn solve(tc: TestCase) {
    let board = Matrix {
        row: tc.n,
        col: tc.n,
        data: tc.board,
    };
    
    let (num_minus_one, num_zero, num_one) = count_square(&board, (0, 0), (tc.n, tc.n));
    println!("{num_minus_one}");
    println!("{num_zero}");
    println!("{num_one}");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().expect("IO error").parse().unwrap();
                                  
    let mut board: Vec<i8> = Vec::new();

    while let Some(line) = lines.next() {
        line.unwrap().split_whitespace().for_each(|c| board.push(c.parse().unwrap()));
    }

    solve(TestCase { n, board });
}