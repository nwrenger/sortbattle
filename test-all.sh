#/bin/bash

hyperfine --warmup 6 -- "target/release/sortbattle nils -s $1 -l $2"
hyperfine --warmup 6 -- "target/release/sortbattle anton -s $1 -l $2"
hyperfine --warmup 6 -- "target/release/sortbattle rust -s $1 -l $2"
hyperfine --warmup 6 -- "target/release/sortbattle rosetta -s $1 -l $2"
