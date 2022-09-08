use std::io::{self, BufRead};

#[derive(Debug)]
enum Command {
    Insert(i32),
    DeleteMax,
    DeleteMin,
}

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    k: usize,
    cmds: Vec<Command>
}

#[derive(Debug)]
struct DoubleHeap {
    n: usize,
    min_heap: Vec<(i32, usize)>,
    max_heap: Vec<(i32, usize)>,
    uid: usize,
    deleted: Vec<bool>,
}

impl DoubleHeap {
    fn new() -> Self {
        DoubleHeap { n: 0, min_heap: Vec::new(), max_heap: Vec::new(), uid: 0, deleted: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn get_max(&mut self) -> Option<i32> {
        if self.n == 0 { return None }

        let mut max = self.extract_top(true);
        while self.deleted[max.1] {
            max = self.extract_top(true);
        }

        return Some(max.0);
    }

    fn get_min(&mut self) -> Option<i32> {
        if self.n == 0 { return None }

        let mut min = self.extract_top(false);
        while self.deleted[min.1] {
            min = self.extract_top(false);
        }

        return Some(min.0);
    }

    fn insert(&mut self, n: i32) {
        self.n += 1;

        self.max_heap.push((n, self.uid));
        let mut idx = self.max_heap.len() - 1;

        while idx > 0 {
            let parent_idx = (idx - 1) / 2;
            if self.max_heap[idx].0 > self.max_heap[parent_idx].0 {
                swap(&mut self.max_heap, idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }

        self.min_heap.push((n, self.uid));
        let mut idx = self.min_heap.len() - 1;

        while idx > 0 {
            let parent_idx = (idx - 1) / 2;
            if self.min_heap[idx].0 < self.min_heap[parent_idx].0 {
                swap(&mut self.min_heap, idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }

        self.deleted.push(false);
        self.uid += 1;
    }

    fn delete_max(&mut self) {
        if self.n == 0 { return; }
        self.n -= 1;

        let mut max = self.extract_top(true);
        while self.deleted[max.1] {
            max = self.extract_top(true);
        }

        // mark deleted uid
        self.deleted[max.1] = true;
    }

    fn delete_min(&mut self) {
        if self.n == 0 { return; }
        self.n -= 1;

        let mut min = self.extract_top(false);
        while self.deleted[min.1] {
            min = self.extract_top(false);
        }

        // mark deleted uid
        self.deleted[min.1] = true;
    }

    fn extract_top(&mut self, is_max: bool) -> (i32, usize) {
        let mut heap = if is_max { &mut self.max_heap } else { &mut self.min_heap };

        // pop the top element of the heap
        let last_idx = heap.len() - 1;
        swap(&mut heap, 0, last_idx);
        let top = heap.pop().unwrap();

        // percolate-down (restoring heap property by top-down order)
        let mut idx = 0;
        while idx < (heap.len() / 2) {
            let l_child_idx = 2 * idx + 1;
            let r_child_idx = 2 * idx + 2;

            let select_left = r_child_idx >= heap.len() 
                            || (is_max && heap[l_child_idx] > heap[r_child_idx]) 
                            || (!is_max && heap[l_child_idx] < heap[r_child_idx]);
            
            let target_idx = if select_left { l_child_idx } else { r_child_idx };
            let need_swap = (is_max && heap[idx] < heap[target_idx]) 
                            || (!is_max && heap[idx] > heap[target_idx]);

            if need_swap {
                swap(&mut heap, idx, target_idx);
                idx = target_idx;
            } else {
                break;
            }
        }

        return top;
    }
}

fn swap(v: &mut Vec<(i32, usize)>, i_a: usize, i_b: usize) {
    let tmp = v[i_a];
    v[i_a] = v[i_b];
    v[i_b] = tmp;
}

fn solve(tc: TestCase) {
    let mut pq = DoubleHeap::new();

    for cmd in tc.cmds {
        match cmd {
            Command::DeleteMin => { pq.delete_min(); },
            Command::DeleteMax => { pq.delete_max(); },
            Command::Insert(n) => { pq.insert(n); },
        }
    }

    if pq.is_empty() {
        println!("EMPTY");
    } else {
        println!("{} {}", pq.get_max().unwrap(), pq.get_min().unwrap());
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().expect("IO error").parse().unwrap();

    for _ in 0..t {
        let k = lines.next().unwrap().expect("IO error").parse().unwrap();
        let mut cmds = Vec::new();

        for _ in 0..k {
            let line = lines.next().unwrap().expect("IO error");

            let mut words = line.split_whitespace();
            let cmd = words.next().unwrap();
            let val: i32 = words.next().unwrap().parse().unwrap();

            match (cmd, val) {
                ("I", n) => cmds.push(Command::Insert(n)),
                ("D", 1) => cmds.push(Command::DeleteMax),
                ("D", -1) => cmds.push(Command::DeleteMin),
                _ => panic!("Wrong command: '{} {}'", cmd, val),
            }
        }

        solve(TestCase { k, cmds });
    }
}