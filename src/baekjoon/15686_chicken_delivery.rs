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

fn minimum_selection(chicken_dists: &Vec<Vec<usize>>, selection: &mut Vec<usize>, start:usize, count: usize) -> usize {
    let selected_count = selection.len();
    if selected_count == count {
        let mut sum = 0;
        let house_count = chicken_dists[0].len();

        for i in 0..house_count {
            let mut min_dist = usize::MAX;
            for &u in selection.iter() {
                min_dist = min_dist.min(chicken_dists[u][i]);
            }

            sum += min_dist;
        }

        return sum;
    }

    let mut min_dist = usize::MAX;
    let shop_count = chicken_dists.len();
    for selected in start..(shop_count - (count - (selected_count + 1))) {
        selection.push(selected);
        min_dist = min_dist.min(minimum_selection(chicken_dists, selection, selected+1, count));
        selection.pop();
    }

    min_dist
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, m, board } = tc;
    let board = Matrix { row: n, col: n, data: board };

    let mut houses = vec![];
    let mut shops = vec![];

    for i in 0..n {
        for j in 0..n {
            match board[(i, j)] {
                1 => houses.push((i, j)),
                2 => shops.push((i, j)),
                _ => continue,
            }
        }
    }

    let chicken_dists = shops.iter().map(|&(u, v)| houses.iter().map(|&(i, j)| u.abs_diff(i) + v.abs_diff(j)).collect()).collect();
    let mut possible_selections = Vec::with_capacity(m);

    minimum_selection(&chicken_dists, &mut possible_selections, 0, m)
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