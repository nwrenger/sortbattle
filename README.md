# sortbattle
A reopsitory with a battle between two quicksort algorithms. One is coded by me and the other one is coded by a good friend of mine. Let's see who wins!

## Dependencies
Only hyperfine hast to be installed using cargo, the other dependencies are stated in the Cargo.toml:
```
cargo install hyperfine
```
## Usage

Make sure to compile the release build of the programm before using the hyperfine command:
```
cargo build -r
```
For my algorithm:
```
hyperfine --warmup 6 -- "target/release/vectest nils -s 69 -l 50000"
```
For my friends algorithm:
```
hyperfine --warmup 6 -- "target/release/vectest anton -s 69 -l 50000"
```
For Rusts algorithm:
```
hyperfine --warmup 6 -- "target/release/vectest rust -s 69 -l 50000"
```
For the algorithm from **[Rosetta Code's Website](https://rosettacode.org/wiki/Sorting_algorithms/Quicksort#Rust)**:
```
hyperfine --warmup 6 -- "target/release/vectest rosetta -s 69 -l 50000"
```
For the generated list(print!):
```
hyperfine --warmup 6 --show-output -- "target/release/vectest unsorted -s 69 -l 50000"
```
with -s you can set the seed for the generated list and with -l you can specify the length of the list.

You can also comment out line 32 and comment in line 33 in main.rs(then save and recompile) for a presorted list. The terminal arguments for comparing the algorithms(hyperfine) don't change. 

And who wins?!?! Compare them yourself!
