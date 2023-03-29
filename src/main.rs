mod algorithms;
use clap::{Parser, ValueEnum};
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_enum)]
    implementation: Implementation,
    #[arg(short, long, default_value_t = 1000)]
    length: usize,
    #[arg(short, long, default_value_t = 0)]
    seed: u64,
}

#[derive(Clone, ValueEnum)]
enum Implementation {
    Unsorted,
    Rust,
    Rosetta,
    Nils,
    Anton,
}

fn main() {
    let Args {
        implementation,
        length,
        seed,
    } = Args::parse();
    let mut unsorted = generate(length, seed);
    // let mut unsorted: Vec<i32> = (0..length as i32).collect();
    // let mut unsorted = vec![2, 1, 3, -4, -6];
    match implementation {
        Implementation::Unsorted => {
            println!("Unsorted: {unsorted:?}");
        }
        Implementation::Rust => {
            unsorted.sort();
            assert!(is_sorted(&unsorted), "{unsorted:?}");
        }
        Implementation::Rosetta => {
            algorithms::rosetta_quicksort(&mut unsorted, &|x, y| x < y);
            assert!(is_sorted(&unsorted), "{unsorted:?}");
        }
        Implementation::Nils => {
            algorithms::nils_quicksort(&mut unsorted);
            assert!(is_sorted(&unsorted), "{unsorted:?}");
        }
        Implementation::Anton => {
            algorithms::anton_quicksort(&mut unsorted);
            assert!(is_sorted(&unsorted), "{unsorted:?}");
        }
    }
}

fn is_sorted(slice: &[i32]) -> bool {
    for pair in slice.windows(2) {
        if pair[0] > pair[1] {
            return false;
        }
    }
    true
}

fn generate(len: usize, seed: u64) -> Vec<i32> {
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(rng.gen());
    }
    vec
}
