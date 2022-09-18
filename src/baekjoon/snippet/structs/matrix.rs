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

impl<T> Matrix<T> {
    fn map_inplace<F>(&mut self, f: F) where F: Fn(&T) -> T {
        for y in 0..self.row {
            for x in 0..self.col {
                self[(y, x)] = f(&self[(y, x)]);
            }
        }
    }

    #[allow(dead_code)]
    fn map<F, U>(&self, f: F) -> Matrix<U> where F: FnMut(&T) -> U {
        let new_data: Vec<U> = self.data.iter().map(f).collect();

        Matrix { row: self.row, col: self.col, data: new_data }
    }

    fn get_neighbor_indexes(&self, idx: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (y, x) = idx;

        if y > 0 { neighbors.push((y-1, x)) }
        if y < self.row-1 { neighbors.push((y+1, x)) }
        if x > 0 { neighbors.push((y, x-1)) }
        if x < self.col-1 { neighbors.push((y, x+1)) }

        return neighbors;
    }
}
