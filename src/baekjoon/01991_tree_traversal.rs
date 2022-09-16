#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    tree: HashMap<char, (Option<char>, Option<char>)>,
}

fn preorder(buf: &mut String, tree: &HashMap<char, (Option<char>, Option<char>)>, node: char) {
    buf.push(node);

    let (l_child, r_child) = tree[&node];
    if let Some(l_child) = l_child {
        preorder(buf, tree, l_child);
    }
    if let Some(r_child) = r_child {
        preorder(buf, tree, r_child);
    }
}

fn inorder(buf: &mut String, tree: &HashMap<char, (Option<char>, Option<char>)>, node: char) {
    let (l_child, r_child) = tree[&node];
    if let Some(l_child) = l_child {
        inorder(buf, tree, l_child);
    }

    buf.push(node);

    if let Some(r_child) = r_child {
        inorder(buf, tree, r_child);
    }
}

fn postorder(buf: &mut String, tree: &HashMap<char, (Option<char>, Option<char>)>, node: char) {
    let (l_child, r_child) = tree[&node];
    if let Some(l_child) = l_child {
        postorder(buf, tree, l_child);
    }
    if let Some(r_child) = r_child {
        postorder(buf, tree, r_child);
    }

    buf.push(node);
}



fn solve(tc: TestCase) -> [String; 3] {
    let TestCase { n, tree } = tc;

    let mut pre_s = String::with_capacity(n);
    preorder(&mut pre_s, &tree, 'A');

    let mut in_s = String::with_capacity(n);
    inorder(&mut in_s, &tree, 'A');

    let mut post_s = String::with_capacity(n);
    postorder(&mut post_s, &tree, 'A');



    [pre_s, in_s, post_s]
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

    let n = lines.next().unwrap().parse().unwrap();
    
    let mut tree = HashMap::new();
    for _ in 0..n {
        let mut chars = lines.next().unwrap().split_whitespace().map(|s| s.chars().next().unwrap());

        let node = chars.next().unwrap();

        let l_child = chars.next().unwrap();
        let l_child = if l_child == '.' { None } else { Some(l_child) };

        let r_child = chars.next().unwrap();
        let r_child = if r_child == '.' { None } else { Some(r_child) };

        tree.insert(node, (l_child, r_child));
    }

    // Solve
    let result = solve(TestCase { n, tree });
    
    for s in result {
        let _ = writeln!(stdout, "{s}");
    }
}