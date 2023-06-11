#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    raw_board: Vec<u8>,
}

#[derive(Debug)]
struct SudokuBoard {
    board: Vec<Option<u8>>
}

#[allow(dead_code)]
impl SudokuBoard {
    fn get_row_iter<'a>(&'a self, row_index: usize) -> impl Iterator<Item = &'a Option<u8>> {
        self.board.iter().skip(row_index * 9).take(9)
    }

    fn get_col_iter<'a>(&'a self, col_index: usize) -> impl Iterator<Item = &'a Option<u8>> {
        self.board.iter().skip(col_index).step_by(9)
    }

    fn get_block_iter<'a>(&'a self, block_index: usize) -> impl Iterator<Item = &'a Option<u8>> {
        let block_start_row = (block_index / 3) * 3; // {0, 1, 2} -> 0, {3, 4, 5} -> 3, {6, 7, 8} -> 6
        let block_start_col = (block_index % 3) * 3; // {0, 3, 6} -> 0, {1, 4, 7} -> 3, {2, 5, 8} -> 6
        let block_start = block_start_row * 9 + block_start_col;

        let block_top = self.board.iter().skip(block_start).take(3);
        let block_middle = self.board.iter().skip(block_start + 9).take(3);
        let block_bottom = self.board.iter().skip(block_start + 9 * 2).take(3);

        block_top.chain(block_middle).chain(block_bottom)
    }

    fn new(raw_board: Vec<u8>) -> Self {
        Self {
            board: raw_board.into_iter().map(|v| if v == 0 { None } else { Some(v) }).collect()
        }
    }

    fn get_next_empty_index(&self, curr_index: usize) -> Option<usize> {
        for i in curr_index..81 {
            if self.board[i] == None { return Some(i) }
        }

        None
    }

    fn get_possible_options(&self, curr_index: usize) -> Vec<u8> {
        let mut is_possible = [true; 10];
        is_possible[0] = false;

        let row_index = curr_index / 9;
        for &cell in self.get_row_iter(row_index) {
            if let Some(v) = cell {
                is_possible[v as usize] = false;
            }
        }

        let col_index = curr_index % 9;
        for &cell in self.get_col_iter(col_index) {
            if let Some(v) = cell {
                is_possible[v as usize] = false;
            }
        }

        let block_index = Self::get_block_index(curr_index);
        for &cell in self.get_block_iter(block_index) {
            if let Some(v) = cell {
                is_possible[v as usize] = false;
            }
        }

        let mut result = Vec::new();
        for i in 1..=9 {
            if is_possible[i as usize] {
                result.push(i);
            }
        }

        result
    }

    fn get_block_index(curr_index: usize) -> usize {
        let row_index = curr_index / 9;
        let col_index = curr_index % 9;

        (row_index / 3) * 3 + (col_index / 3)
    }

    fn solve(&mut self) -> bool {
        let next_empty = self.get_next_empty_index(0);
        match next_empty {
            Some(idx) => self.solve_inner(idx),
            None => true,
        }
    }

    fn solve_inner(&mut self, curr_index: usize) -> bool {
        let options = self.get_possible_options(curr_index);
        let next_empty = self.get_next_empty_index(curr_index + 1);
        for i in options {
            self.board[curr_index] = Some(i);

            if next_empty.is_none() || self.solve_inner(next_empty.unwrap()) { return true; }
        }

        self.board[curr_index] = None;
        false
    }
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_index in 0..9 {
            let row_str = self.get_row_iter(row_index)
                .map(|v| char::from_digit(v.unwrap_or(0_u8) as u32, 10).unwrap())
                .collect::<String>();

            writeln!(f, "{}", row_str)?;
        }

        Ok(())
    }
}

fn solve(tc: TestCase) -> SudokuBoard {
    let TestCase { raw_board } = tc;
    let mut board = SudokuBoard::new(raw_board);

    if board.solve() == false {
        panic!("Given Sudoku board is unsolvable!");
    }

    board
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
    let inputs = buffer.chars().filter(|c| !c.is_whitespace()).map(|c| c.to_digit(10).unwrap() as u8);

    let raw_board = inputs.collect();

    // Solve
    let result = solve(TestCase { raw_board });
    
    let _ = writeln!(stdout, "{result}");
}