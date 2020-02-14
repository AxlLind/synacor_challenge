use std::collections::HashMap;

type Cache = HashMap<(u16,u16),u16>;

/*
  Below is the assembly that runs the expensive computation.
  To the left are comments as to what each instruction does.

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

  We see that the register $0 is the only register used after
  the function returns so this is clearly the return value.
  After some analysis, we can convert this into the function f below.

  It is a very slow Ackermann-like function, but with memoization
  it is fast enough to find the correct setting of 25734 with in
  a reasonable amount of time.
*/

fn f(cache: &mut Cache, args: (u16,u16), c: u16) -> u16 {
  if let Some(&v) = cache.get(&args) {
    return v;
  }
  let v = match args {
    (0,y) => y+1,
    (x,0) => f(cache, (x-1, c), c),
    (x,y) => {
      let y = f(cache, (x, y-1), c);
      f(cache, (x-1, y), c)
    }
  };
  cache.insert(args, v);
  v
}

fn compute_setting(c: u16) -> u16 {
  f(&mut Cache::new(), (4,1), c)
}

fn main() {
  let v = (25730..0x8000)
    .find(|&c| compute_setting(c) == 6)
    .unwrap();
  println!("f(4,1,{}) = 6", v);
}
