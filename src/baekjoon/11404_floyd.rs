#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::{Index, IndexMut};
use std::fmt;

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

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.row {
            for j in 0..self.col {
                if j == 0 {
                    write!(f, "{}", self[(i, j)])?;
                } else {
                    write!(f, " {}", self[(i, j)])?;
                }
            }

            if i < self.row - 1 {
                write!(f, "\n")?;
            }
        }
    
        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    edges: Vec<(usize, usize, usize)>,
}

fn solve(tc: TestCase) -> Matrix<usize> {
    let inf: usize = 2_000_000_000;

    let TestCase { n, edges, .. } = tc;
    let mut dist = Matrix {
        row: n,
        col: n,
        data: vec![inf; n*n],
    };

    // initialize distance matrix by edges
    for i in 0..n {
        dist[(i, i)] = 0;
    }
    for (i, j, c) in edges {
        dist[(i, j)] = dist[(i, j)].min(c);
    }

    // Floyd-Warshall
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[(i, j)] = dist[(i, j)].min(dist[(i, k)] + dist[(k, j)]);
            }
        }
    }

    // replace unreachable distances (inf) to 0
    for v in dist.data.iter_mut() {
        if *v == inf {
            *v = 0;
        }
    }

    dist
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

    let mut edges = vec![];
    for _ in 0..m {
        let a = inputs.next().unwrap() - 1;
        let b = inputs.next().unwrap() - 1;
        let c = inputs.next().unwrap();

        edges.push((a, b, c));
    }

    // Solve
    let result = solve(TestCase { n, m, edges });
    
    let _ = writeln!(stdout, "{result}");
}