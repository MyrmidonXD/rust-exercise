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
    board: Vec<Option<u8>>,
    row_status: [[bool; 10]; 9],
    col_status: [[bool; 10]; 9],
    block_status: [[bool; 10]; 9],
}

#[allow(dead_code)]
impl SudokuBoard {
    fn new(raw_board: Vec<u8>) -> Self {
        let mut sudoku_board = Self { 
            board: raw_board.into_iter().map(|v| if v == 0 { None } else { Some(v) }).collect(),
            row_status: [[false; 10]; 9],
            col_status: [[false; 10]; 9],
            block_status: [[false; 10]; 9]
        };

        for i in 0..sudoku_board.board.len() {
            if let Some(v) = sudoku_board.board[i] {
                sudoku_board.update_status(i, v, true);
            }
        }

        sudoku_board
    }

    fn get_row_index(curr_index: usize) -> usize {
        curr_index / 9
    }

    fn get_col_index(curr_index: usize) -> usize {
        curr_index % 9
    }

    fn get_block_index(curr_index: usize) -> usize {
        let row_index = Self::get_row_index(curr_index);
        let col_index = Self::get_col_index(curr_index);

        (row_index / 3) * 3 + (col_index / 3)
    }

    fn get_next_empty_index(&self, curr_index: usize) -> Option<usize> {
        for i in curr_index..81 {
            if self.board[i] == None { return Some(i) }
        }

        None
    }

    fn set(&mut self, curr_index: usize, new_value: u8) {
        if let None = self.board[curr_index] {
            self.board[curr_index] = Some(new_value);
            self.update_status(curr_index, new_value, true);
        }
    }

    fn reset(&mut self, curr_index: usize) {
        if let Some(old_value) = self.board[curr_index] {
            self.board[curr_index] = None;
            self.update_status(curr_index, old_value, false);
        }
    }

    fn update_status(&mut self, curr_index: usize, target_value: u8, is_set: bool) {
        let row_index = Self::get_row_index(curr_index);
        self.row_status[row_index][target_value as usize] = is_set;

        let col_index = Self::get_col_index(curr_index);
        self.col_status[col_index][target_value as usize] = is_set;

        let block_index = Self::get_block_index(curr_index);
        self.block_status[block_index][target_value as usize] = is_set;
    }

    fn is_valid_at(&self, curr_index: usize, new_value: u8) -> bool {
        let row_index = Self::get_row_index(curr_index);
        let col_index = Self::get_col_index(curr_index);
        let block_index = Self::get_block_index(curr_index);
    
        !self.row_status[row_index][new_value as usize]
            && !self.col_status[col_index][new_value as usize]
            && !self.block_status[block_index][new_value as usize]
    }

    fn solve(&mut self) -> bool {
        let next_empty = self.get_next_empty_index(0);
        match next_empty {
            Some(idx) => self.solve_inner(idx),
            None => true,
        }
    }

    fn solve_inner(&mut self, curr_index: usize) -> bool {
        let next_empty = self.get_next_empty_index(curr_index + 1);
        for v in 1..=9 {
            if self.is_valid_at(curr_index, v) {
                self.set(curr_index, v);

                if next_empty.is_none() || self.solve_inner(next_empty.unwrap()) { return true; }

                self.reset(curr_index);
            }
        }

        false
    }
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_iter = self.board.iter();
        for _ in 0..9 {
            let row_str = board_iter.by_ref().take(9)
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