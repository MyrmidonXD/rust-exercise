#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    r: usize,
    c: usize,
    t: usize,
    board: Vec<i32>,
}

fn diffuse(board: &Matrix<i32>) -> Matrix<i32> {
    let mut result = Matrix {
        row: board.row,
        col: board.col,
        data: vec![0; board.row * board.col],
    };

    for i in 0..board.row {
        for j in 0..board.col {
            if board[(i, j)] == -1 {
                result[(i, j)] = -1;
                continue;
            }

            let mut a = board[(i, j)];
            let diff_amount = a / 5;

            if i > 0 && board[(i-1, j)] != -1 {
                result[(i-1, j)] += diff_amount;
                a -= diff_amount;
            }
            if i < board.row - 1 && board[(i+1, j)] != -1 {
                result[(i+1, j)] += diff_amount;
                a -= diff_amount;
            }
            if j > 0 && board[(i, j-1)] != -1 {
                result[(i, j-1)] += diff_amount;
                a -= diff_amount;
            }
            if j < board.col - 1 { // board[(i, j+1)] cannot be -1
                result[(i, j+1)] += diff_amount;
                a -= diff_amount;
            }

            result[(i, j)] += a;
        }
    }

    result
}

fn circulate(board: &mut Matrix<i32>, order: &[(usize, usize)]) {
    let mut prev = order[0];

    for curr in &order[1..] {
        board[prev] = board[*curr];
        prev = *curr;
    }

    board[prev] = 0;
}

fn solve(tc: TestCase) -> usize {
    let TestCase { r, c, t, board } = tc;
    let mut board = Matrix {
        row: r,
        col: c,
        data: board,
    };

    let mut upper_circulator = 0;
    for i in 0..r {
        if board[(i, 0)] == -1 {
            upper_circulator = i;
            break;
        }
    }
    let lower_circulator = upper_circulator + 1;

    let mut upper_cycle = vec![];
    for i in (0..upper_circulator).rev() {
        upper_cycle.push((i, 0));
    }
    for j in 1..c {
        upper_cycle.push((0, j));
    }
    for i in 1..=upper_circulator {
        upper_cycle.push((i, c-1));
    }
    for j in (1..c-1).rev() {
        upper_cycle.push((upper_circulator, j));
    }

    let mut lower_cycle = vec![];
    for i in lower_circulator+1..r {
        lower_cycle.push((i, 0));
    }
    for j in 1..c {
        lower_cycle.push((r-1, j));
    }
    for i in (lower_circulator..r-1).rev() {
        lower_cycle.push((i, c-1));
    }
    for j in (1..c-1).rev() {
        lower_cycle.push((lower_circulator, j));
    }

    for _ in 0..t {
        board = diffuse(&board);
        circulate(&mut board, &upper_cycle);
        circulate(&mut board, &lower_cycle);
    }

    (board.data.into_iter().sum::<i32>() + 2) as usize
}

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut buffer = String::new();

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    // Parsing
    let _ = stdin.read_to_string(&mut buffer);
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let r = inputs.next().unwrap() as usize;
    let c = inputs.next().unwrap() as usize;
    let t = inputs.next().unwrap() as usize;

    let board = inputs.collect();

    // Solve
    let result = solve(TestCase { r, c, t, board });
    
    let _ = writeln!(stdout, "{result}");
}