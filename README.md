# Synacor Challenge
Solution to the [Synacor Challenge](https://challenge.synacor.com/) in Rust.

:warning: Major spoilers in the next section. Avoid reading this unless you want the challenge spoiled.

## Approach
This details roughly the steps I took to solve the challenge and the different strategies I did along the way.

1. `Build the VM`. Not too much issue here, took maybe 2-3 hours with some debugging to get it working. Having done [Advent of Code 2019](https://github.com/AxlLind/AdventOfCode2019/) certainly helped here though. I think my implementation ended up quite clean. See [cpu.rs](./src/cpu.rs).
2. `Manually explore`. I started manually exploring the game. Found the can after some frustration in the maze. Started writing down the commands I did and automatically feeding it to the CPU at the start of the program. This meant I did not have to replay the beginning all the time. Manually exploring worked up until you encounter the locked door. See [inputs.txt](./inputs.txt).
3. `Bruteforcing coin order`. After exploring the game you end up with 5 coins at a locked door. You need to place them in the correct order. The description of the coins gives hints to their value and the door gives you an equation. So the puzzle is to satisfy the equation with the order we place the coins. With 5 coins there are only `5! = 120` permutations to check so this can easily be brute forced. See [solution here](./src/bin/solve_coins.rs).
