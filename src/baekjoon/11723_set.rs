use std::io::{self, BufRead, BufReader, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdin = BufReader::new(stdin.lock());

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut lines = stdin.lines().skip(1);

    let mut set = [0usize; 20];
    while let Some(line) = lines.next() {
        let query = line.unwrap();
        let query: Vec<&str> = query.split(' ').collect();
        
        let op = query[0];
        let x: usize = query.get(1).map(|c| c.parse().unwrap()).unwrap_or(21) - 1;
        match op {
            "add" => { set[x] = 1; },
            "remove" => { set[x] = 0; },
            "check" => { let _ = writeln!(stdout, "{}", set[x]); },
            "toggle" => { set[x] = 1 - set[x]; },
            "all" => { set = [1usize; 20]; },
            "empty" => { set = [0usize; 20]; },
            _ => panic!("Wrong op `{}`", op),
        }
    }

}