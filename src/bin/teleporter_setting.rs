/*
  0x178b:  jt $0 0x1793      # if $0 != 0 { goto 0x1793; }
  0x178e: add $0 $1 0x1      # $0 = $1 + 1
  0x1792: ret                # return
  0x1793:  jt $1 0x17a0      # if $1 != 0 { goto 0x17a0; }
  0x1796: add $0 $0 0x7fff   # $0 -= 1
  0x179a: set $1 $7          # $1 = $7
  0x179d: cll 0x178b         # 0x178b()
  0x179f: ret                # return
  0x17a0: psh $0             # stack.push($0)
  0x17a2: add $1 $1 0x7fff   # $1 -= 1
  0x17a6: cll 0x178b         # 0x178b()
  0x17a8: set $1 $0          # $1 = $0
  0x17ab: pop $0             # $0 = stack.pop()
  0x17ad: add $0 $0 0x7fff   # $0 -= 1
  0x17b1: cll 0x178b         # 0x178b()
  0x17b3: ret                # return

  This is the assembly that runs the expensive computation. To the
  right are comments I added to make it easier for me to analyze.
  We see that the register $0 is the only register used after the
  function returns so this is clearly the single return value.

  After some careful analysis, we can convert this into the
  function f below. We see that this is a very slow Ackermann-like
  function, but adding memoization turns out to make it fast enough.

  Note that the value of 'a' never increases. This means the
  possible values of (a,b) is relatively small. We can thus
  use a much faster array as a cache instead of a HashMap.
  This turned out to be about an 8x speed up.

  We can also note some relations in the Ackermann function:
    - f(1,b) = b+1+c,
    - f(2,b) = b*(c+1) + 2*c + 1
  With these optimizations the program finds the correct
  setting in about 10 seconds!
*/

type Cache = [[Option<u16>; 0x10000]; 5];

fn f(cache: &mut Cache, c: u16, a: u16, b: u16) -> u16 {
  let (_a,_b) = (a as usize, b as usize);
  if let Some(v) = cache[_a][_b] { return v; }

  let v = match (a,b) {
    (0,b) => b+1,
    (1,b) => b+1+c,
    (2,b) => b*(c+1) + 2*c + 1,
    (a,0) => f(cache, c, a-1, c),
    (a,b) => {
      let b = f(cache, c, a, b-1);
      f(cache, c, a-1, b)
    }
  };

  cache[_a][_b] = Some(v);
  v
}

fn valid_setting(&c: &u16) -> bool {
  let mut cache = [[None; 0x10000]; 5];
  f(&mut cache, c, 4, 1) == 6
}

fn main() {
  let setting = (0..0x8000).find(valid_setting).unwrap();
  println!("f({}) = 6", setting);
}
