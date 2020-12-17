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
      --------Part 1--------   --------Part 2--------
Day       Time   Rank  Score       Time   Rank  Score
 16   03:01:59   9269      0   19:29:29  19024      0
 15   00:56:28   5490      0   01:05:24   4357      0
 14   00:28:39   2649      0   01:31:20   3793      0
 13   00:44:13   6478      0   02:18:52   3633      0
 12   01:02:03   6399      0   01:17:58   4862      0
 11   01:33:24   6993      0   01:56:05   5598      0
 10   00:32:43   7576      0   01:24:04   4486      0
  9   00:34:48   7399      0   01:12:31   8310      0
  8   00:28:09   6728      0   01:15:44   7449      0
  7   01:18:07   6640      0   01:38:11   5461      0
  6   00:18:24   6310      0   00:44:57   7242      0
  5   00:18:31   3302      0   00:33:14   4303      0
  4   01:26:50  11141      0   02:35:22  10274      0
  3   00:26:40   5913      0   00:51:32   7172      0
  2   00:27:55   5471      0   00:44:20   5928      0
  1   00:13:29   2444      0   00:17:24   2444      0
```