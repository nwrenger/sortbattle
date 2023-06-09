# sortbattle
A reopsitory with a battle between two quicksort algorithms. One is coded by me and the other one is coded by a good friend of mine. Let's see who wins!

## Dependencies
Only hyperfine has to be installed using cargo, the other dependencies are stated in the Cargo.toml:
```
cargo install hyperfine
```
Alternatively hyperfine can also be installed with a package manager e.g. `sudo pacman -Syu hyperfine`.

## Usage
Make sure to Compile the release build of the programm before using the hyperfine command. You only have to Compile the Programm again when you made changes to the code:
```
cargo build -r
```
For my algorithm:
```
hyperfine --warmup 6 -- "target/release/sortbattle nils -s 69 -l 50000"
```
For my friends algorithm:
```
hyperfine --warmup 6 -- "target/release/sortbattle anton -s 69 -l 50000"
```
For Rusts algorithm:
```
hyperfine --warmup 6 -- "target/release/sortbattle rust -s 69 -l 50000"
```
For the algorithm from **[Rosetta Code's Website](https://rosettacode.org/wiki/Sorting_algorithms/Quicksort#Rust)**:
```
hyperfine --warmup 6 -- "target/release/sortbattle rosetta -s 69 -l 50000"
```
The Makefile should execute all the above commands (if rust is installed)
```
make test-all
```
The shell script only executes the test cases. The first argument to the script is the seed, the second the length of the
test array 
```
./test-all.sh 69 50000
```

For the generated list(print!):
```
hyperfine --warmup 6 --show-output -- "target/release/sortbattle unsorted -s 69 -l 50000"
```
with -s you can set the seed for the generated list and with -l you can specify the length of the list.

You can also comment out line 32 and comment in line 33 in main.rs(then save and recompile) for a presorted list. The terminal arguments for comparing the algorithms(hyperfine) don't change. 

And who wins?!?! Compare them yourself!
