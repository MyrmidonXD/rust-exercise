#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::fmt;
use std::cmp::PartialEq;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    initial_board: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
struct Board {
    n: usize,
    cells: Vec<Vec<usize>> 
}

#[allow(dead_code)]
impl Board {
    fn up(&self) -> Self {
        let mut next_board = self.clone();

        let n = next_board.n;
        for col_index in 0..n {
            let mut col_nonzero: Vec<usize> = next_board.cells
                .iter()
                .map(|row| row[col_index])
                .filter(|&x| x != 0)
                .collect();
            
            Board::merge_front(&mut col_nonzero);

            for row_index in 0..n {
                if row_index < col_nonzero.len() {
                    next_board.cells[row_index][col_index] = col_nonzero[row_index];
                } else {
                    next_board.cells[row_index][col_index] = 0;
                }
            }
        }

        next_board
    }

    fn down(&self) -> Self {
        let mut next_board = self.clone();

        let n = next_board.n;
        for col_index in 0..n {
            let mut col_nonzero: Vec<usize> = next_board.cells
                .iter()
                .map(|row| row[col_index])
                .filter(|&x| x != 0)
                .collect();

            Board::merge_back(&mut col_nonzero);

            for row_index in 0..n {
                if row_index < n - col_nonzero.len() {
                    next_board.cells[row_index][col_index] = 0;
                } else {
                    next_board.cells[row_index][col_index] = col_nonzero[row_index - (n - col_nonzero.len())];
                }
            }
        }

        next_board
    }

    fn left(&self) -> Self {
        let mut next_board = self.clone();

        let n = next_board.n;
        for row_index in 0..n {
            let mut row_nonzero: Vec<usize> = next_board.cells[row_index]
                .iter()
                .cloned()
                .filter(|&x| x != 0)
                .collect();

            Board::merge_front(&mut row_nonzero);

            for col_index in 0..n {
                if col_index < row_nonzero.len() {
                    next_board.cells[row_index][col_index] = row_nonzero[col_index];
                } else {
                    next_board.cells[row_index][col_index] = 0;
                }
            }
        }

        next_board
    }

    fn right(&self) -> Self {
        let mut next_board = self.clone();

        let n = next_board.n;
        for row_index in 0..n {
            let mut row_nonzero: Vec<usize> = next_board.cells[row_index]
                .iter()
                .cloned()
                .filter(|&x| x != 0)
                .collect();

            Board::merge_back(&mut row_nonzero);

            for col_index in 0..n {
                if col_index < n - row_nonzero.len() {
                    next_board.cells[row_index][col_index] = 0;
                } else {
                    next_board.cells[row_index][col_index] = row_nonzero[col_index - (n - row_nonzero.len())];
                }
            }
        }

        next_board
    }

    fn merge_front(list: &mut [usize]) {
        let mut target_index = 0;
        let mut check_index = 0;

        while check_index < list.len() {
            if check_index == list.len() - 1 || list[check_index] != list[check_index + 1]{
                list[target_index] = list[check_index];
                check_index += 1;
            } else {
                list[target_index] = list[check_index + 1] * 2;
                list[check_index + 1] = 0;
                check_index += 2;
            }

            target_index += 1;
        }

        while target_index < list.len() {
            list[target_index] = 0;
            target_index += 1;
        }
    }

    fn merge_back(list: &mut [usize]) {
        list.reverse();
        Board::merge_front(list);
        list.reverse();
    }

    fn find_max(&self) -> usize {
        *self.cells
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max().unwrap()
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        if self.n != other.n { return false; }

        for row_index in 0..self.n {
            for col_index in 0..self.n {
                if self.cells[row_index][col_index] != other.cells[row_index][col_index] {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row_index in 0..self.n {
            let row_str = self.cells[row_index]
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join("\t");

            writeln!(f, "{}", row_str)?;
        }

        Ok(())
    }
}

fn find_possible_max(board: Board, remaining_move: usize) -> usize {
    if remaining_move == 0 {
        return board.find_max();
    }

    let next_boards = IntoIterator::into_iter([
        board.up(),
        board.down(),
        board.left(),
        board.right(),
    ]);

    next_boards
        .filter(|b| b != &board)
        .fold(board.find_max(), |max, b| max.max(find_possible_max(b, remaining_move - 1)))
}

fn solve(tc: TestCase) -> usize {
    let board = Board {
        n: tc.n,
        cells: tc.initial_board,
    };

    find_possible_max(board, 5)
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
    let mut initial_board = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = Vec::with_capacity(n);
        for _ in 0..n {
            row.push(inputs.next().unwrap());
        }
        initial_board.push(row);
    }

    // Solve
    let result = solve(TestCase { n, initial_board });
    
    let _ = writeln!(stdout, "{result}");
}