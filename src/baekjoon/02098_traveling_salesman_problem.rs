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
    costs: Vec<u32>,
}

fn solve(tc: TestCase) -> u32 {
    let TestCase { n, costs } = tc;
    let costs = Matrix {
        row: n,
        col: n,
        data: costs,
    };

    let masks: Vec<u32> = (0..n).map(|i| 1 << i).collect();
    let end = (1 << n) - 1;

    let mut sets = vec![vec![]; n+1];
    for i in 0..=end as u32 {
        let mut elem_count = 0;
        for &mask in &masks {
            if i & mask > 0 {
                elem_count += 1;
            }
        }

        sets[elem_count].push(i);
    }

    let mut min_cost = vec![vec![u32::MAX; n]; 1 << n];
    for &s in &sets[2] {
        if s & 1 > 0  {
            for j in 1..n {
                if s & masks[j] > 0 && costs[(0, j)] > 0 {
                    min_cost[s as usize][j] = costs[(0, j)];
                    break;
                }
            }
        }
    }

    for i in 3..=n {
        let sets = &sets[i];

        for &s in sets {
            for k in 1..n {
                if s & masks[k] > 0 {
                    let rest = s - masks[k];

                    for j in 1..n {
                        if min_cost[rest as usize][j] != u32::MAX && costs[(j, k)] != 0 {
                            min_cost[s as usize][k] = min_cost[s as usize][k].min(min_cost[rest as usize][j] + costs[(j, k)]);
                        }
                    }
                }
            }
        }
    }

    let mut min = u32::MAX;
    for i in 1..n {
        if min_cost[end][i] != u32::MAX && costs[(i, 0)] != 0 {
            min = min.min(min_cost[end][i] + costs[(i, 0)]);
        }
    }

    min
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<u32>().unwrap());

    let n = inputs.next().unwrap() as usize;
    let costs = inputs.collect();

    // Solve
    let result = solve(TestCase { n, costs });
    
    let _ = writeln!(stdout, "{result}");
}