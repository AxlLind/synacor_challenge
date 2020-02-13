use itertools::Itertools;

/*
  we are given coins with values 2,3,5,7,9
  and this equation to solve:
    _ + _ * _^2 + _^3 - _ = 399
  only 5! = 120 combinations to check so
  lets just brute force a solution
*/
pub fn brute_force_coins() -> Vec<&'static str> {
  [2,3,5,7,9].iter()
    .permutations(5)
    .find(|p| p[0] + p[1]*p[2]*p[2] + p[3]*p[3]*p[3] - p[4] == 399)
    .unwrap()
    .iter()
    .map(|i| match i {
      2 => "use red coin",
      3 => "use corroded coin",
      5 => "use shiny coin",
      7 => "use concave coin",
      9 => "use blue coin",
      _ => panic!()
    })
    .collect()
}
