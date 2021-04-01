# Benchmarking

This submodule is housing all of the benchmarking code.

The benchmarking works as follows:

- Create x amount of random i32 vectors of size y
- Clone these vectors for every function we want to benchmark
- Run the modified functions which are benchmark ready on all the cloned vectors
- Calculate the averages
- Print the results

x can be changed by passing a value with "-a"

y can be changed by passing a value with "-l"

for example:
```bash
bench.exe -a 5 -l 20000
```

## mod.rs

The entry point for the bench binary resides in mod.rs.

## benched_sort.rs etc

These files contain modified implementations of the algorithms in the main module. They are not generic (they only accept i32) and they count their swap operations and their compare operations.