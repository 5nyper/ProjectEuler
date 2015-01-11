use std::iter::AdditiveIterator;
use std::num::Float;

fn is_prime(x: u64) -> bool {
    if x < 2 { return false; }
    if x == 2 { return true; }
    for i in 2..((x as f64).sqrt().ceil() as u64 + 1) {
        if x % i == 0 { return false; }
    }
    true
}

fn main() {
    println!("{}", (2u64..2_000_000).filter(|&x| is_prime(x)).sum());
}
