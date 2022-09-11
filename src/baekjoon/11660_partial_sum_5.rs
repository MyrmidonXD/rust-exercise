#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    table: Vec<usize>,
    queries: Vec<((usize, usize), (usize, usize))>,
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

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match index {
            (y, _) if y >= self.row => { panic!("index out of bounds: the row size is {} but the row index is {}", self.row, y); },
            (_, x) if x >= self.col => { panic!("index out of bounds: the column size is {} but the column index is {}", self.col, x); }
            (y, x) => { return &mut self.data[y * self.col + x]; }
        }
    }
}

fn solve(tc: TestCase) -> Vec<usize> {
    // TODO: Implement this
    let table = Matrix { row: tc.n, col: tc.n, data: tc.table };
    let mut acc_table = Matrix { row: tc.n + 1, col: tc.n + 1, data: vec![0; (tc.n + 1) * (tc.n + 1)] };

    for x in 1..=tc.n {
        for y in 1..=tc.n {
            acc_table[(x, y)] = table[(x-1, y-1)] + acc_table[(x-1, y)] + acc_table[(x, y-1)] - acc_table[(x-1, y-1)]
        }
    }
    
    let mut results = vec![];
    for ((x1, y1), (x2, y2)) in tc.queries {
        let area_sum = acc_table[(x2, y2)] + acc_table[(x1-1, y1-1)] - acc_table[(x1-1, y2)] - acc_table[(x2, y1-1)];
        results.push(area_sum);
    }

    results
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

    let mut table = vec![];
    for _ in 0..n*n {
        table.push(inputs.next().unwrap());
    }

    let mut queries = vec![];
    for _ in 0..m {
        let x1 = inputs.next().unwrap();
        let y1 = inputs.next().unwrap();
        let x2 = inputs.next().unwrap();
        let y2 = inputs.next().unwrap();

        queries.push(((x1, y1), (x2, y2)));
    }

    // Solve
    let results = solve(TestCase { n, m, table, queries });
    for result in results {
        let _ = writeln!(stdout, "{result}");
    }
}