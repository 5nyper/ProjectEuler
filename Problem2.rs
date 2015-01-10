use std::iter::Iterator;
use std::iter::AdditiveIterator;

struct Fib(u32, u32);

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let r = self.0;
        self.0 = self.1;
        self.1 += r;
        Some(r)
    }
}

fn main() {
    println!("{}", Fib(1, 2).filter(|&x| x % 2 == 0).take_while(|&x| x < 4_000_000).sum());
}
