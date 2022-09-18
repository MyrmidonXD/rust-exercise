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
    n: usize,
    m: usize,
    board: Vec<usize>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, m, board } = tc;
    let mut board = Matrix { row: n, col: m, data: board };

    let mut cheese_count = board.data.iter().filter(|&&x| x == 1).count();
    let mut elapsed = 0;

    let mut target = vec![];
    let mut visited = Matrix { row: n, col: m, data: vec![0u8; n*m] };
    let mut stack = vec![];

    while cheese_count > 0 {
        // reset visited
        for x in visited.data.iter_mut() {
            *x = 0;
        }

        visited[(0, 0)] = 1;
        stack.push((0, 0));

        // dfs
        while let Some((y, x)) = stack.pop() {
            // up
            if y > 0 {
                let next = (y-1, x);
                if board[next] == 1 {
                    visited[next] += 1;
                    if visited[next] == 2 { target.push(next) };
                } else if visited[next] == 0 { // = (board[next] == 0 && visited[next] == 0)
                    visited[next] = 1;
                    stack.push(next);
                }
            }
            // down
            if y < board.row - 1 {
                let next = (y+1, x);
                if board[next] == 1 {
                    visited[next] += 1;
                    if visited[next] == 2 { target.push(next) };
                } else if visited[next] == 0 { // = (board[next] == 0 && visited[next] == 0)
                    visited[next] = 1;
                    stack.push(next);
                }
            }
            // left
            if x > 0 {
                let next = (y, x-1);
                if board[next] == 1 {
                    visited[next] += 1;
                    if visited[next] == 2 { target.push(next) };
                } else if visited[next] == 0 { // = (board[next] == 0 && visited[next] == 0)
                    visited[next] = 1;
                    stack.push(next);
                }
            }
            // right
            if x < board.col - 1 {
                let next = (y, x+1);
                if board[next] == 1 {
                    visited[next] += 1;
                    if visited[next] == 2 { target.push(next) };
                } else if visited[next] == 0 { // = (board[next] == 0 && visited[next] == 0)
                    visited[next] = 1;
                    stack.push(next);
                }
            }
        }

        // melt target cheeses
        while let Some(cheese) = target.pop() {
            board[cheese] = 0;
            cheese_count -= 1;
        }

        elapsed += 1;
    }

    elapsed
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