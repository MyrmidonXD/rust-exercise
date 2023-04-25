#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    k: usize,
    t1: Vec<usize>,
    t2: Vec<usize>,
}

/*
    아이디어 정리

    문제에서 제시된 집합 S를 구하는 함수 f(t1, t2) 가 있다고 하자.
    임의의 leaf node가 아닌 t1, t2에 대해, 이 함수 f(t1, t2)가 어떤 일을 해야 하는가?

    t1의 왼쪽 서브트리를 l1, 오른쪽 서브트리를 r1이라 하자.
    t2의 왼쪽 서브트리를 l2, 오른쪽 서브트리를 r2이라 하자.
             
            t1                       t2
          /    \                   /    \
        /\      /\               /\      /\
       /  \    /  \             /  \    /  \
      / l1 \  / r1 \           / l2 \  / r2 \
      ------  ------           ------  ------

    왼쪽 서브트리는 왼쪽 서브트리끼리, 오른쪽 서브트리는 오른쪽 서브트리끼리 따져본다고 생각해보자.
    f(l1, l2) 를 해서 나온 결과가 { p1, p2, p3, p4 },
    f(r1, r2) 를 해서 나온 결과가 { q1, q2 } 라면,
    p1 -> q1의 경로는 항상 현재 t1, t2의 루트 노드를 지나야 한다. p2 -> q1, p3 -> q1, p4 -> q1 도 마찬가지다.
    p1 -> q2 또한 마찬가지고, p2 -> q2, p3 -> q2, p4 -> q2 모두 t1, t2 의 루트 노드를 지나가야 한다.

    따라서 p* -> q* 는 t1, t2 양쪽에서 모두 거리가 같다.
    그리고 p* 끼리, q* 끼리는 이미 각 서브트리 내에서 T1, T2 내 거리가 같은 노드들이었기 때문에,
    f(t1, t2) 의 가능한 결과 중 하나는 { p1, p2, p3, p4, q1, q2 } 가 된다.

    한편, 한 트리의 왼쪽 서브트리와 다른 트리의 오른쪽 서브트리를 매치시켜서 따져보는 경우도 가능하다.
    f(l1, r2) 를 해서 나온 결과가 { a1, a2, a3, a4 },
    f(r1, l2) 을 해서 나온 결과가 { b1, b2, b3, b4 } 라면
    위의 케이스와 같은 논리에 따라
    f(t1, t2) 의 가능한 결과 중 하나로 { a1, a2, a3, a4, b1, b2, b3, b4 } 를 얻을 수 있다.

    따라서 두 결과중 크기가 더 큰 후자를 선택해서 반환하면 된다.

    그렇다면 이러한 재귀 구조의 Base Case는 어떻게 정의되는가?
    두 트리가 전부 leaf node 인 경우를 따져보면, 서브트리가 존재하지 않기 때문에 위처럼 두가지 케이스로 분기되는 것은 아닐 것이다.

    두 leaf node에 할당된 값이 같다면 그 자신을 담은 집합, 다르다면 공집합을 반환한다고 생각해보자.
    그렇다면 level 2인 t1, t2를 생각해서 따져볼 수 있다.
    
     t1      t2
     /\      /\
    1  2    2  1

    이런 경우에는 방향 맞춰서 매칭시켰을 때 양쪽 다 공집합 -> 결과도 공집합이 된다.
    방향 다르게 매칭시켰을 때는 한쪽은 { 1 }, 다른 쪽은 { 2 } 가 되어 결과가 { 1, 2 } 가 된다.
    따라서 f(t1, t2)는 { 1, 2 } 를 반환하게 될 것이다.

    두 트리의 leaf node들이 서로 다른 케이스도 따져보자.
     
     t1      t2
     /\      /\
    1  3    7  1

    정방향 매칭일 땐 공집합, 역방향 매칭일 땐 { 1 } 이 결과값이 된다. 따라서 f(t1, t2) 는 { 1 } 을 반환한다.
    홀수개가 나오는 게 조금 꺼림칙하긴 했지만, 상위 level에서를 생각해보면 { 1 } 과 다른 쪽 서브트리의 집합의 원소들끼린 경로가 전부 root node를 지나게 될 것이다.
    이런 방식으로 위에서 따졌던 방식을 적용시킬 수 있다.
    따라서 이러한 Base Case 정의가 괜찮다는 것을 확인하였다.

    이제 함수 f의 정의에서 집합을 반환할 필요가 있는가? 를 생각해 보았는데,
    결국 문제에서 구하고자 하는 건 이 최대 집합 S의 원소의 갯수 뿐이다.
    { p1, p2, p3, p4 } 와 { q1, q2 } 가 각각 구해졌을 때, 저 p1, p2, p3, p4가 각각 무엇인지 중요한가?
    p1, p2, p3, p4 와 q1, q2는 당연히 서로 다른 값들이고, 다른 방향 매칭에서의 a1, a2, a3, a4와 p1, p2, p3, p4는 절대 같을 수 없다.
    (같은 원소가 있다면 그 말은 그 원소가 l2와 r2에 동시에 존재한다는 말이 된다.)

    따라서 이 집합의 원소가 무엇인가? 는 중요하지 않다.
    두 결과 중 큰 쪽의 갯수를 반환할 때, 매칭의 방향 또한 선택되어 상위 레벨로 올라간다고 생각해도 될 것 같다.

    그래서 결국 아래 구현대로 간단한 재귀함수로 구현이 끝날 수 있었다.
 */

fn get_maximum_set_size(t1: &[usize], t2: &[usize]) -> usize {
    if t1.len() != t2.len() { return 0; }

    if t1.len() == 1 {
        return if t1[0] == t2[0] { 1 } else { 0 }
    }

    let mid = t1.len() / 2;
    let (t1_left, t1_right) = (&t1[0..mid], &t1[mid..t1.len()]);
    let (t2_left, t2_right) = (&t2[0..mid], &t2[mid..t2.len()]);

    let normal_result = get_maximum_set_size(t1_left, t2_left) + get_maximum_set_size(t1_right, t2_right);
    let flipped_result = get_maximum_set_size(t1_left, t2_right) + get_maximum_set_size(t1_right, t2_left);

    return normal_result.max(flipped_result);
}

fn solve(tc: TestCase) -> usize {
    let TestCase { t1, t2, .. } = tc;
    get_maximum_set_size(&t1[..], &t2[..])
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

    let k = inputs.next().unwrap();
    let t1 = inputs.by_ref().take(2_usize.pow((k-1) as u32)).collect();
    let t2 = inputs.collect();

    // Solve
    let result = solve(TestCase { k, t1, t2 });
    
    let _ = writeln!(stdout, "{result}");
}