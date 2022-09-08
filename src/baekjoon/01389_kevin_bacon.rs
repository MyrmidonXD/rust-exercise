use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::convert::{TryInto};

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: usize,
    m: usize,
    edges: Vec<(usize, usize)>
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

fn solve(tc: TestCase) {
    let mut distance_matrix = Matrix {
        row: tc.n,
        col: tc.n,
        data: vec![usize::MAX; tc.n * tc.n],
    };

    // initialize the distance matrix
    for i in 0..tc.n {
        distance_matrix[(i, i)] = 0;
    }
    for (a, b) in tc.edges {
        distance_matrix[(a-1, b-1)] = 1;
        distance_matrix[(b-1, a-1)] = 1;
    }

    // Floyd-Warshall
    for k in 0..tc.n {
        for i in 0..tc.n {
            for j in 0..tc.n {
                if distance_matrix[(i, j)] > distance_matrix[(i, k)].saturating_add(distance_matrix[(k, j)]) {
                    distance_matrix[(i, j)] = distance_matrix[(i, k)].saturating_add(distance_matrix[(k, j)]);
                }
            }
        }
    }

    let mut min = usize::MAX;
    let mut min_idx = 0;

    for i in 0..tc.n {
        let mut sum = 0;
        for j in 0..tc.n {
            sum += distance_matrix[(i, j)];
        }

        if sum < min {
            min = sum;
            min_idx = i;
        }
    }

    println!("{}", min_idx+1);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let [n, m]: [usize; 2] = lines.next().unwrap().expect("IO error").split_whitespace()
                                    .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                                    .try_into().unwrap();

    let mut edges = Vec::new();
    for _ in 0..m {
        let [a, b]: [usize; 2] = lines.next().unwrap().expect("IO error").split_whitespace()
                                    .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                                    .try_into().unwrap();
        
        edges.push((a, b));
    }
    
    solve(TestCase { n, m, edges });
}