#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::{Index, IndexMut};
use std::collections::VecDeque;

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
    n: usize,
    m: usize,
    board: Vec<usize>,
}

fn get_safe_area(board: &Matrix<usize>, viruses: &Vec<(usize, usize)>, visited: &mut Matrix<bool>, queue: &mut VecDeque<(usize, usize)>) -> usize {
    for &(i, j) in viruses {
        if visited[(i, j)] { continue; }

        queue.push_back((i, j));
        visited[(i, j)] = true;

        while let Some((i, j)) = queue.pop_front() {
            // up
            if i > 0 && board[(i-1, j)] != 1 && !visited[(i-1, j)] {
                queue.push_back((i-1, j));
                visited[(i-1, j)] = true;
            }
            // down
            if i < board.row - 1 && board[(i+1, j)] != 1 && !visited[(i+1, j)] {
                queue.push_back((i+1, j));
                visited[(i+1, j)] = true;
            }
            // left
            if j > 0 && board[(i, j-1)] != 1 && !visited[(i, j-1)] {
                queue.push_back((i, j-1));
                visited[(i, j-1)] = true;
            }
            // right
            if j < board.col - 1 && board[(i, j+1)] != 1 && !visited[(i, j+1)] {
                queue.push_back((i, j+1));
                visited[(i, j+1)] = true;
            }
        }
    }

    let mut count = 0;
    for i in 0..board.row {
        for j in 0..board.col {
            if board[(i, j)] == 0 && !visited[(i, j)] {
                count += 1;
            }
        }
    }

    count
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, m, board } = tc;

    let mut board = Matrix {
        row: n,
        col: m,
        data: board,
    };

    let mut empties = vec![];
    let mut viruses = vec![];

    for i in 0..n {
        for j in 0..m {
            match board[(i, j)] {
                0 => empties.push((i, j)),
                2 => viruses.push((i, j)),
                _ => continue,
            }
        }
    }

    // pre-allocate structures for BFS
    // these can be reused between multiple BFS calls
    let mut visited = Matrix {
        row: board.row,
        col: board.col,
        data: vec![false; board.row * board.col],
    };
    let mut queue = VecDeque::new();

    let mut max_area = 0;
    let empties_count = empties.len();
    for i in 0..empties_count-2 {
        // set 3 walls
        board[empties[i]] = 1;
        for j in i+1..empties_count-1 {
            board[empties[j]] = 1;
            for k in j+1..empties_count {
                board[empties[k]] = 1;

                max_area = max_area.max(get_safe_area(&board, &viruses, &mut visited, &mut queue));

                // reset visited matrix
                for v in &mut visited.data {
                    *v = false;
                }

                // reset 3 walls
                board[empties[k]] = 0;
            }
            board[empties[j]] = 0;
        }
        board[empties[i]] = 0;
    }

    max_area
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    let n = inputs.next().unwrap();
    let m = inputs.next().unwrap();
    let board = inputs.collect();

    // Solve
    let result = solve(TestCase { n, m, board });
    
    let _ = writeln!(stdout, "{result}");
}