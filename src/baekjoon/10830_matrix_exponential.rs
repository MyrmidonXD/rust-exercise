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

use std::fmt;

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

impl Matrix<usize> {
    fn mult_mod(a: &Matrix<usize>, b: &Matrix<usize>, m: usize) -> Matrix<usize> {
        if a.col != b.row {
            panic!(
                "Dimension error: Two matrices of size `{}x{}` and `{}x{}` cannot be multiplicated!",
                a.row,
                a.col,
                b.row,
                b.col
            );
        }

        let mut result = Matrix {
            row: a.row,
            col: b.col,
            data: vec![0; a.row*b.col],
        };

        for i in 0..result.row {
            for j in 0..result.col {
                for k in 0..a.col {
                    result[(i, j)] = (result[(i, j)] + ((a[(i, k)] * b[(k, j)]) % m)) % m;
                }
            }
        }

        result
    }

    fn exp_mod(a: &Matrix<usize>, n: usize, m: usize) -> Matrix<usize> {
        if n == 1 {
            let mut result = a.clone();
            for v in &mut result.data {
                *v = *v % m;
            }

            return result;
        } else if n == 2 {
            return Self::mult_mod(a, a, m);
        } else if n == 3 {
            let square = Self::mult_mod(a, a, m);
            return Self::mult_mod(&square, a, m);
        } else {
            let root = Self::exp_mod(a, n/2, m);

            let mut result = Self::mult_mod(&root, &root, m);
            if n % 2 == 1 {
                result = Self::mult_mod(&result, a, m);
            }

            return result;      
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    b: usize,
    mat: Vec<usize>,
}

fn solve(tc: TestCase) -> Matrix<usize> {
    let TestCase { n, b, mat } = tc;
    let a = Matrix { row: n, col: n, data: mat };

    Matrix::exp_mod(&a, b, 1000)
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
    let b = inputs.next().unwrap();
    let mat = inputs.collect();

    // Solve
    let result = solve(TestCase { n, b, mat });
    
    let _ = writeln!(stdout, "{result}");
}