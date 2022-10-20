#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Clone, Copy)]
struct Segment {
    l: Point,
    r: Point,
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Segment {
    fn new((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> Self {
        Segment {
            l: Point { x: x1, y: y1 },
            r: Point { x: x2, y: y2 },
        }
    }

    fn is_vertical(&self) -> bool {
        self.l.x == self.r.x
    }

    fn get_y_at_x(&self, x: i32) -> (Option<i32>, bool) {
        if self.is_vertical() || x < self.l.x || x > self.r.x {
            return (None, false);
        }

        let x_diff = self.r.x - self.l.x;
        let y_diff = self.r.y - self.l.y;
        let dx = x - self.l.x;

        let mut y = self.l.y + ((dx as i64 * y_diff as i64) / x_diff as i64) as i32;
        let is_precise = (dx as i64 * y_diff as i64) % x_diff as i64 == 0;

        if !is_precise && y_diff < 0 {
            // y must be the largest integer below the segment, 
            // but integer division for negative dividend (dx * y_diff < 0) gives the smallest integer above the segment.
            // so it should be decremented by one.
            y -= 1;
        }

        (Some(y), is_precise)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    l1: Vec<(i32, i32)>,
    l2: Vec<(i32, i32)>,
}

fn solve(tc: TestCase) -> bool {
    let TestCase { mut l1, mut l2 } = tc;

    l1.sort_unstable();
    l2.sort_unstable();

    let (l_left, l_right) = if l1[0].0 <= l2[0].0 { (l1, l2) } else { (l2, l1) };

    let seg_left = Segment::new(l_left[0], l_left[1]);
    let seg_right = Segment::new(l_right[0], l_right[1]);

    if seg_left.is_vertical() && seg_right.is_vertical() {
        if seg_left.l.x != seg_right.l.x { return false; }

        let (seg_down, seg_up) = if seg_left.l.y <= seg_right.l.y {
            (seg_left, seg_right)
        } else {
            (seg_right, seg_left)
        };

        return seg_down.r.y >= seg_up.l.y;
    } else if seg_left.is_vertical() || seg_right.is_vertical() {
        let (seg_vert, seg_diag) = if seg_left.is_vertical() { (seg_left, seg_right) } else { (seg_right, seg_left) };

        let vert_x = seg_vert.l.x;
        if seg_diag.l.x > vert_x || seg_diag.r.x < vert_x { return false; }

        let (y, is_precise) = seg_diag.get_y_at_x(vert_x);
        let y = y.unwrap();

        return (is_precise && y >= seg_vert.l.y && y <= seg_vert.r.y) || (!is_precise && y >= seg_vert.l.y && y+1 <= seg_vert.r.y);
    } else {
        if seg_left.r.x < seg_right.l.x { return false; }

        let (y1, is_precise) = seg_left.get_y_at_x(seg_right.l.x);
        let y1 = y1.unwrap();

        let seg_right_l_on_seg_left = is_precise && seg_right.l.y == y1;
        let seg_right_l_below_seg_left = (is_precise && seg_right.l.y < y1) || (!is_precise && seg_right.l.y <= y1);
        let seg_right_l_above_seg_left = (is_precise && seg_right.l.y > y1) || (!is_precise && seg_right.l.y >= y1+1);

        if seg_right.r.x <= seg_left.r.x {
            let (y2, is_precise) = seg_left.get_y_at_x(seg_right.r.x);
            let y2 = y2.unwrap();

            let seg_right_r_on_seg_left = is_precise && seg_right.r.y == y2;
            let seg_right_r_below_seg_left = (is_precise && seg_right.r.y < y2) || (!is_precise && seg_right.r.y <= y2);
            let seg_right_r_above_seg_left = (is_precise && seg_right.r.y > y2) || (!is_precise && seg_right.r.y >= y2+1);

            return seg_right_l_on_seg_left
                || seg_right_r_on_seg_left
                || (seg_right_l_below_seg_left && seg_right_r_above_seg_left) 
                || (seg_right_l_above_seg_left && seg_right_r_below_seg_left);
        } else {
            let (y3, is_precise) = seg_right.get_y_at_x(seg_left.r.x);
            let y3 = y3.unwrap();

            let seg_left_r_on_seg_right = is_precise && seg_left.r.y == y3;
            let seg_left_r_below_seg_right = (is_precise && seg_left.r.y < y3) || (!is_precise && seg_left.r.y <= y3);
            let seg_left_r_above_seg_right = (is_precise && seg_left.r.y > y3) || (!is_precise && seg_left.r.y >= y3+1);

            return seg_right_l_on_seg_left
                || seg_left_r_on_seg_right
                || (seg_right_l_below_seg_left && seg_left_r_below_seg_right)
                || (seg_right_l_above_seg_left && seg_left_r_above_seg_right);
        }
    }
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let p1 = (inputs.next().unwrap(), inputs.next().unwrap());
    let p2 = (inputs.next().unwrap(), inputs.next().unwrap());
    let p3 = (inputs.next().unwrap(), inputs.next().unwrap());
    let p4 = (inputs.next().unwrap(), inputs.next().unwrap());

    // Solve
    let result = solve(TestCase { l1: vec![p1, p2], l2: vec![p3, p4] });
    
    let _ = writeln!(stdout, "{}", if result { 1 } else { 0 });
}