use std::collections::{HashSet, VecDeque};

/*
  We are in a temple that looks like this:
    *  8  -  1
    4  *  11 *
    +  4  -  18
    22 -  9  *
  You start at the 22 room and the goal of the puzzle is
  to end up at the 1 room with a "value" of 30. You start
  with a value of 22 and moving in the temple updates it
  with the corresponding operation and value of the rooms.

  This is just a creative path finding problem. We need to
  find the shortest path and since this is an unweighted
  graph BFS is sufficient. No need for Dijkstra's algorithm.

  The key idea is to each node in the graph is a pair (room,score),
  not just a room. This avoids infinite loops in the BFS while at
  the same time allowing you to visit a room more than once.
  This means we start at node (6,22) and the goal is simply (1,30).
*/

#[derive(Debug,Copy,Clone)]
enum Op { Add, Sub, Mul }

struct Node { val: i32, neighbours: Vec<(usize,Op)> }

fn bfs(g: &[Node], (start,score): (usize, i32), end: (usize, i32)) -> Vec<(usize,Op)> {
  let mut queue = VecDeque::new();
  let mut visited = HashSet::new();
  queue.push_back(vec![(start,score,Op::Add)]);

  let path = loop {
    let path = queue.pop_front().unwrap();
    let &(u,score,_) = path.last().unwrap();

    if (u,score) == end { break path; }
    visited.insert((u,score));

    let neighbours = g[u].neighbours.iter()
      .map(|&(v,op)| {
        let score = match op {
          Op::Add => score + g[v].val,
          Op::Sub => score - g[v].val,
          Op::Mul => score * g[v].val,
        };
        (v,score,op)
      })
      .filter(|&(v,score,_)| !visited.contains(&(v,score)));
    for v in neighbours {
      let mut new_path = path.clone();
      new_path.push(v);
      queue.push_back(new_path);
    }
  };

  path.iter()
    .skip(1)
    .map(|&(v,_,op)| (v,op))
    .collect()
}

fn main() {
  // translated by hand from the maze above
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

  // Printing (node,op) makes you able to find the commands in
  // the game that make up the path. That seemed easier than
  // writing code to automatically output the path as commands.
  for v in bfs(&graph, (6,22), (1,30)) { println!("{:?}", v); }
}
