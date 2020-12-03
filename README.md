# adventofcode2020
These are my, [Niklas Hallqvist](https://github.com/niklasha) solutions to
[Advent of code 2020](https://adventofcode.com/2020).
They are written in [Rust](https://rust-lang.org).

My reason for doing these are, besides the fact that I like puzzle solving, I want to improve my skills in Rust.

You need Rust, [rustup](https://rustup.rs/) is the suggested way to install Rust, that is about it.  You may need to add some SSL libraries, depending on operating system, but the installation process will tell you, if so.

Run all the days with:
```
cargo run input/
```

Where "input/" is a prefix for the days' inputs, named 01, 02, etc.
The tests (the examples given in the days' descriptions) can be run with:
```
cargo test
```

When I solved a puzzle in a not so idiomatic or nice way, and later went back to refine the code, I have chosen to retain the naive solution as dead code with method names suffixed with  "_naive".

```
My results were:
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  3   00:26:40  5913      0   00:51:32  7172      0
  2   00:27:55  5471      0   00:44:20  5928      0
  1   00:13:29  2444      0   00:17:24  2444      0
```