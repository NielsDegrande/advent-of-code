# Advent of Code

Versioning code and data artifacts related to [Advent of Code](https://adventofcode.com/).

Code here is not representative for real work.

## Setup

To set up a new day, run `YEAR=XXXX DAY=XX make initialize_day`.
To run a day:

```shell
cd YEAR/dayXX
cargo build --release
target/release/dayXX
```
