#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::Index;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    board: Vec<usize>,
}

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

fn compress_area(board: &Matrix<usize>, start: (usize, usize), end: (usize, usize)) -> String {
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
        return color.to_string();
    }

    let (ym, xm) = ((y1+y2)/2, (x1+x2)/2);

    format!(
        "({}{}{}{})",
        compress_area(board, (y1, x1), (ym, xm)),
        compress_area(board, (y1, xm), (ym, x2)),
        compress_area(board, (ym, x1), (y2, xm)),
        compress_area(board, (ym, xm), (y2, x2)),
    )
}

fn solve(tc: TestCase) -> String {
    let board = Matrix { row: tc.n, col: tc.n, data: tc.board };

    compress_area(&board, (0, 0), (tc.n, tc.n))
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
    let mut lines = buffer.lines();

    let n = lines.next().unwrap().parse().unwrap();
    let mut board = vec![];

    while let Some(s) = lines.next() {
        let chars = s.chars();
        chars.for_each(|c| board.push(c.to_digit(10).unwrap() as usize));
    }

    // Solve
    let result = solve(TestCase { n, board });

    let _ = writeln!(stdout, "{result}");
}