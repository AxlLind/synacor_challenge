use std::collections::{HashSet, VecDeque};

#[derive(Debug,Copy,Clone)]
enum Op { Add, Sub, Mul }

struct Node { val: i64, neighbours: Vec<(usize,Op)> }

/*
  Key idea: view your state as (place,score), not just the place.
  This avoids infinite loops while at the same time allowing you
  to visit a square more than once. This means the state we want
  to reach is simply (1,30) and we start at (6,22).

  Since this is an unweighted graph BFS is good enough to find
  the shortest path, we don't need Dijkstra's algorithm.
*/
fn bfs(g: &[Node], (start,score): (usize, i64), end: (usize, i64)) -> Vec<(usize,i64,Op)> {
  let mut queue = VecDeque::new();
  let mut visited = HashSet::new();
  queue.push_back(vec![(start,score,Op::Add)]);

  loop {
    let path = queue.pop_front().unwrap();
    let &(u,score,_) = path.last().unwrap();

    if (u,score) == end { break path; }
    visited.insert((u,score));

    g[u].neighbours.iter()
      .map(|&(v,op)| {
        let score = match op {
          Op::Add => score + g[v].val,
          Op::Sub => score - g[v].val,
          Op::Mul => score * g[v].val,
        };
        (v,score,op)
      })
      .filter(|&(v,score,_)| !visited.contains(&(v,score)))
      .for_each(|v| {
        let mut new_path = path.clone();
        new_path.push(v);
        queue.push_back(new_path);
      });
  }
}

/*
  The given maze:
    *  8  -  1
    4  *  11 *
    +  4  -  18
    22 -  9  *
  Translated by hand into the graph below
*/
fn main() {
  let graph = [
    Node { val: 8,  neighbours: vec![(1,Op::Sub),(3,Op::Sub),(2,Op::Mul),(4,Op::Mul),(3,Op::Mul)] },
    Node { val: 1,  neighbours: vec![(0,Op::Sub),(3,Op::Sub),(5,Op::Mul),(3,Op::Mul)] },
    Node { val: 4,  neighbours: vec![(0,Op::Mul),(4,Op::Mul),(3,Op::Mul),(4,Op::Add)] },
    Node { val: 11, neighbours: vec![(0,Op::Sub),(1,Op::Sub),(4,Op::Sub),(5,Op::Sub),(7,Op::Sub),(0,Op::Mul),(1,Op::Mul),(2,Op::Mul),(4,Op::Mul),(5,Op::Mul)] },
    Node { val: 4,  neighbours: vec![(2,Op::Mul),(0,Op::Mul),(3,Op::Mul),(2,Op::Add),(7,Op::Sub),(3,Op::Sub),(5,Op::Sub)] },
    Node { val: 18, neighbours: vec![(1,Op::Mul),(3,Op::Mul),(7,Op::Mul),(4,Op::Sub),(7,Op::Sub),(3,Op::Sub)] },
    Node { val: 22, neighbours: vec![(2,Op::Add),(4,Op::Add),(4,Op::Sub),(7,Op::Sub)] },
    Node { val: 9,  neighbours: vec![(4,Op::Sub),(6,Op::Sub),(3,Op::Sub),(5,Op::Sub),(5,Op::Mul)] },
  ];

  /*
    I print the Op to be able to by hand find the commands
    that make up the path. That seemed easier than writing
    code to automatically output the path as commands.
  */
  for (v,_,op) in bfs(&graph, (6,22), (1,30)).iter().skip(1) {
    println!("{} {:?}", v, op)
  }
}
