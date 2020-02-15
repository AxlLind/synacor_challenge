use std::collections::HashMap;

/*
  Below is the assembly that runs the expensive computation. To the
  left are comments I made to make it easier to understand for me.

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
  the function returns so this is clearly the single return value.
  After some analysis, we can convert this into the function f below.

  We see that this is a very slow Ackermann-like function,
  but with memoization it is fast enough. Still slow but
  finds the correct setting of 25734 in about 7 minutes.
*/

fn f(cache: &mut HashMap<(u16,u16),u16>, c: u16, args: (u16,u16)) -> u16 {
  if let Some(&v) = cache.get(&args) { return v; }

  let v = match args {
    (0,b) => b+1,
    (a,0) => f(cache, c, (a-1,c)),
    (a,b) => {
      let b = f(cache, c, (a,b-1));
      f(cache, c, (a-1,b))
    }
  };

  cache.insert(args, v);
  v
}

fn main() {
  let mut cache = HashMap::new();
  let v = (0..0x8000).find(|&c| {
    cache.clear();
    f(&mut cache, c, (4,1)) == 6
  });
  println!("f(4,1,{}) = 6", v.unwrap());
}
