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
    h: usize,
    w: usize,
    board: Vec<char>,
    keys: Vec<char>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { h, w, board, keys } = tc;
    
    let mut has_key = [false; 26];
    if keys.len() > 1 || keys[0] != '0' {
        for c in keys {
            let key_index = ((c as u32) - ('a' as u32)) as usize;
            has_key[key_index] = true;
        }
    }

    let board = Matrix {
        row: h,
        col: w,
        data: board,
    };

    let mut visited = Matrix {
        row: h,
        col: w,
        data: vec![false; h*w],
    };

    let mut starting_points = Vec::with_capacity(2*(h+w));
    for i in [0, h-1] {
        for j in 0..w {
            if board[(i, j)] != '*' {
                starting_points.push(Some((i, j)));
            }
        }
    }
    for i in 1..h-1 {
        for j in [0, w-1] {
            if board[(i, j)] != '*' {
                starting_points.push(Some((i, j)));
            }
        }
    }

    let mut neighbors = Vec::with_capacity(4);
    let mut result = 0;
    let mut queue = VecDeque::with_capacity(h*w);
    loop {
        for p in &mut starting_points {
            match *p {
                Some((i, j)) => {
                    let c = board[(i, j)];
                    
                    let is_empty = c == '.';
                    let is_key = c as u32 >= 'a' as u32 && c as u32 <= 'z' as u32;
                    let is_openable_door = c as u32 >= 'A' as u32 && c as u32 <= 'Z' as u32 && has_key[(c as u32 - 'A' as u32) as usize];
                    let is_doc = c == '$';

                    if is_key {
                        let key_index = ((c as u32) - ('a' as u32)) as usize;
                        has_key[key_index] = true;
                    }

                    let accessible = is_empty || is_key || is_openable_door || is_doc;
                    if accessible {
                        *p = None;

                        if !visited[(i, j)] {
                            if is_doc {
                                result += 1;
                            }

                            queue.push_back((i, j));
                            visited[(i, j)] = true;
                        }
                    }
                },
                None => continue,
            }
        }

        if queue.is_empty() { break; }

        while let Some((i, j)) = queue.pop_front() {
            if i > 0 {
                neighbors.push((i-1, j));
            }
            if i < h-1 {
                neighbors.push((i+1, j));
            }
            if j > 0 {
                neighbors.push((i, j-1));
            }
            if j < w-1 {
                neighbors.push((i, j+1));
            }

            for &next in &neighbors {
                if !visited[next] {
                    let c = board[next];

                    let mut accessible = c == '.';
                    if c == '$' {
                        result += 1;
                        accessible = true;
                    } else if c as u32 >= 'a' as u32 && c as u32 <= 'z' as u32 {
                        let key_index = ((c as u32) - ('a' as u32)) as usize;
                        has_key[key_index] = true;
                        accessible = true;
                    } else if c as u32 >= 'A' as u32 && c as u32 <= 'Z' as u32 {
                        let key_index = ((c as u32) - ('A' as u32)) as usize;
                        accessible = has_key[key_index];

                        if !accessible {
                            starting_points.push(Some(next));
                        }
                    }

                    if accessible {
                        queue.push_back(next);
                        visited[next] = true;
                    }
                }
            }

            neighbors.clear();
        }
    }

    result
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

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut xs = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());

        let h = xs.next().unwrap();
        let w = xs.next().unwrap();

        let mut board = Vec::with_capacity(h*w);
        for _ in 0..h {
            let mut line = lines.next().unwrap().chars().take(w);

            while let Some(c) = line.next() {
                board.push(c);
            }
        }

        let line = lines.next().unwrap();
        let keys = line.chars().collect();

        // Solve
        let result = solve(TestCase { h, w, board, keys });
    
        let _ = writeln!(stdout, "{result}");
    }
}