//Idiomatic Code
#![feature(slicing_syntax)]

use std::iter::AdditiveIterator;

fn main() {
    let lastnum = 1000;
    // +1 since we specify the index one beyond the end in range notation
    let range = 0..(lastnum + 1);
    // filter creates an iterator that leaves out stuff divisible by 3 or 5
    let v = range.filter(|&x| x%5 == 0 || x%3 == 0);
    println!("{}", v.sum());
}
