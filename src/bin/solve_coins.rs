use itertools::Itertools;

/*
  we are given coins with values 2,3,5,7,9
  and this equation to solve:
    _ + _ * _^2 + _^3 - _ = 399
  only 5! = 120 combinations to check so
  this can easily be brute forced
*/

fn main() {
  let answer = [2,3,5,7,9].iter()
    .permutations(5)
    .find(|p| p[0] + p[1]*p[2]*p[2] + p[3]*p[3]*p[3] - p[4] == 399)
    .unwrap();
  for i in answer {
    match i {
      2 => println!("use red coin"),
      3 => println!("use corroded coin"),
      5 => println!("use shiny coin"),
      7 => println!("use concave coin"),
      9 => println!("use blue coin"),
      _ => panic!(),
    }
  }
}
