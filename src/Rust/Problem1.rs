//Idiomatic Code
use std::iter::AdditiveIterator;
fn main() {
    println!("{}", (0u32..1000).filter(|&x| (x % 3 == 0) || (x % 5 == 0)).sum());
}
