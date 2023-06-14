#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    board: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum BoardStatus {
    Playing((usize, usize), (usize, usize)),
    Fail,
    Success
}

#[derive(Debug)]
struct Board {
    row: usize,
    column: usize,
    board: Vec<Vec<char>>,
    status: Option<BoardStatus>,
}

impl Board {
    fn new(board: Vec<Vec<char>>) -> Self {
        let mut board = Board {
            row: board.len(),
            column: board[0].len(),
            board: board,
            status: None,
        };

        let (red, blue) = board.get_bead_positions().unwrap();
        
        // remove beads on the board itself
        board.board[red.0][red.1] = '.';
        board.board[blue.0][blue.1] = '.';

        // save bead positions in the status instead
        board.apply_bead_positions(BoardStatus::Playing(red, blue));

        board
    }

    fn get_bead_positions(&self) -> Option<((usize, usize), (usize, usize))> {
        if let Some(BoardStatus::Playing(red, blue)) = self.status {
            return Some((red, blue));
        } else if let Some(_) = self.status {
            return None;
        }

        let mut result = ((0, 0), (0, 0));

        for row_index in 0..self.row {
            for col_index in 0..self.column {
                if self.board[row_index][col_index] == 'R' {
                    result.0 = (row_index, col_index);
                } else if self.board[row_index][col_index] == 'B' {
                    result.1 = (row_index, col_index);
                }
            }
        }

        Some(result)
    }

    fn apply_bead_positions(&mut self, next_status: BoardStatus) {
        if let BoardStatus::Playing(_, _) = next_status {
            self.status = Some(next_status);
        }
    }

    fn flip(&self, (row_delta, col_delta): (isize, isize)) -> BoardStatus {
        let (red, blue) = self.get_bead_positions().unwrap();

        let mut end_state = None;
        let mut fix_red = false;

        let mut new_red = red;
        loop {
            let next = (add(new_red.0, row_delta) , add(new_red.1, col_delta));
            match self.board[next.0][next.1] {
                '#' => { break; },
                'O' => { end_state = Some(true); break; },
                _ => {
                    // if the red ball 'crosses over' the blue ball at this point,
                    // then the red ball must be one cell 'behind' from the blue ball at the end.
                    if next == blue {
                        fix_red = true; 
                    };
                    new_red = next;
                }
            }
        };

        let mut new_blue = blue;
        loop {
            let next = (add(new_blue.0, row_delta), add(new_blue.1, col_delta));
            match self.board[next.0][next.1] {
                '#' => { break; },
                'O' => { end_state = Some(false); break; },
                _ => { new_blue = next; }
            }
        }

        if let Some(success) = end_state {
            return if success { BoardStatus::Success } else { BoardStatus::Fail };
        }

        if new_red == new_blue {
            if fix_red {
                new_red = (add(new_red.0, -row_delta), add(new_red.1, -col_delta));
            } else {
                new_blue = (add(new_blue.0, -row_delta), add(new_blue.1, -col_delta));
            }
        }

        BoardStatus::Playing(new_red, new_blue)
    }
}

#[inline]
fn add(lhs: usize, rhs: isize) -> usize {
    if rhs < 0 {
        lhs.checked_sub(rhs.unsigned_abs()).unwrap()
    } else {
        lhs + rhs.unsigned_abs()
    }
}

fn solve(tc: TestCase) -> i32 {
    let TestCase { board, .. } = tc;
    let mut board = Board::new(board);

    let (initial_red, initial_blue) = board.get_bead_positions().unwrap();
    let initial_state = BoardStatus::Playing(initial_red, initial_blue);

    let mut visited = HashSet::new();
    visited.insert(initial_state);
    visited.insert(BoardStatus::Fail);

    let mut queue = VecDeque::new();
    let next_states = IntoIterator::into_iter([
        board.flip((0, -1)),
        board.flip((0, 1)),
        board.flip((-1, 0)),
        board.flip((1, 0)),
    ]);

    for state in next_states {
        if state == BoardStatus::Success {
            return 1; // early return before bfs
        }

        if !visited.contains(&state) {
            visited.insert(state);
            queue.push_back((state, 1));
        }
    }

    // Breadth-First Search
    while let Some((state, num_move)) = queue.pop_front() {
        if num_move >= 10 { break; }

        board.apply_bead_positions(state);
        let next_states = IntoIterator::into_iter([
            board.flip((0, -1)),
            board.flip((0, 1)),
            board.flip((-1, 0)),
            board.flip((1, 0)),
        ]);

        for state in next_states {
            if state == BoardStatus::Success {
                return num_move + 1;
            }

            if !visited.contains(&state) {
                visited.insert(state);
                queue.push_back((state, num_move + 1));
            }
        }
    }

    -1
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
    let mut inputs = buffer.split_whitespace();

    let n = inputs.next().unwrap().parse().unwrap();
    let m = inputs.next().unwrap().parse().unwrap();
    
    let mut board = Vec::with_capacity(n);
    for _ in 0..n {
        let row = inputs.next().unwrap().chars().collect::<Vec<char>>();
        board.push(row);
    }

    // Solve
    let result = solve(TestCase { n, m, board });
    
    let _ = writeln!(stdout, "{result}");
}