## day05 using `rustc` / LLVM for optimization

This solution is split into two binaries.

- `compiler` takes a text file with a sample or input from aoc and produces a rust representation of the input
- `solver` then calls into the generated rust code
- when `solver` is compiled in release mode, `rustc` optimizes the naive code for us

In my testing, this is enough to solve this problem on a single M1 core in 1:30:

```
~/repos/private/aoc2023/day05/solver time target/release/solver
11611182
target/release/solver  90.70s user 0.01s system 99% cpu 1:30.99 total
```
