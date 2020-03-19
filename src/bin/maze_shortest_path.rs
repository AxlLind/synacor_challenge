use std::collections::{HashSet, VecDeque};

/*
  We are in a temple that looks like this:
    *  8  -  1
    4  *  11 *
    +  4  -  18
    22 -  9  *
  You start at the 22 room and the goal of the puzzle is to end
  up at the 1 room with a "value" of 30. You start with a value
  of 22 and moving in the temple updates it with the corresponding
  operation and value of the rooms.

  This is just a creative path-finding problem. We need to find
  the shortest path and since this is an unweighted graph BFS
  is sufficient. No need for Dijkstra's algorithm.

  The key idea is to each node in the graph is a pair (room,score),
  not just a room. This avoids infinite loops in the BFS while at
  the same time allowing you to visit a room more than once.
  This means we start at node (6,22) and the goal is simply (1,30).

  22 + 4 - 11 * 4 - 18 - 11 - 1
  The program outputs the path as an expression. This makes you able
  to find the corresponding set of game commands by hand. That seemed
  simpler than writing code to automatically output the path as commands.
*/

static G: [Node; 8] = [
  Node { val: 8,  neighbours: &[(1,Op::Sub),(3,Op::Sub),(2,Op::Mul),(4,Op::Mul),(3,Op::Mul)] },
  Node { val: 1,  neighbours: &[(0,Op::Sub),(3,Op::Sub),(5,Op::Mul),(3,Op::Mul)] },
  Node { val: 4,  neighbours: &[(0,Op::Mul),(4,Op::Mul),(3,Op::Mul),(4,Op::Add)] },
  Node { val: 11, neighbours: &[(0,Op::Sub),(1,Op::Sub),(4,Op::Sub),(5,Op::Sub),(7,Op::Sub),(0,Op::Mul),(1,Op::Mul),(2,Op::Mul),(4,Op::Mul),(5,Op::Mul)] },
  Node { val: 4,  neighbours: &[(2,Op::Mul),(0,Op::Mul),(3,Op::Mul),(2,Op::Add),(7,Op::Sub),(3,Op::Sub),(5,Op::Sub)] },
  Node { val: 18, neighbours: &[(1,Op::Mul),(3,Op::Mul),(7,Op::Mul),(4,Op::Sub),(7,Op::Sub),(3,Op::Sub)] },
  Node { val: 22, neighbours: &[(2,Op::Add),(4,Op::Add),(4,Op::Sub),(7,Op::Sub)] },
  Node { val: 9,  neighbours: &[(4,Op::Sub),(6,Op::Sub),(3,Op::Sub),(5,Op::Sub),(5,Op::Mul)] },
];

#[derive(Clone,Copy)]
enum Op { Add, Sub, Mul }

struct Node {
  val: i32,
  neighbours: &'static [(usize,Op)],
}

fn bfs((start,score): (usize, i32), goal: (usize, i32)) -> Vec<(usize,Op)> {
  let mut queue = VecDeque::new();
  let mut visited = HashSet::new();
  queue.push_back(vec![(start,Op::Add,score)]);

  let path = loop {
    let path = queue.pop_front().unwrap();
    let &(u,_,score) = path.last().unwrap();

    if (u,score) == goal { break path; }
    visited.insert((u,score));

    for &(v,op) in G[u].neighbours {
      let score = match op {
        Op::Add => score + G[v].val,
        Op::Sub => score - G[v].val,
        Op::Mul => score * G[v].val,
      };
      if visited.contains(&(v,score)) {
        continue;
      }
      let mut new_path = path.clone();
      new_path.push((v,op,score));
      queue.push_back(new_path);
    }
  };

  path.iter().map(|&(v,op,_)| (v,op)).collect()
}

fn main() {
  let path = bfs((6,22), (1,30));
  let expr = path.iter()
    .map(|&(v,op)| match op {
      Op::Add => format!(" + {}", G[v].val),
      Op::Sub => format!(" - {}", G[v].val),
      Op::Mul => format!(" * {}", G[v].val),
    })
    .collect::<String>();
  println!("{}", &expr[3..]);
}
