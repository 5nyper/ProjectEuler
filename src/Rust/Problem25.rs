fn main() {
println!("fib(10000) = {}", fib(4782));
}
// oli-obk's Code
extern crate num;

use num::{one, pow};
use num::bigint::{ToBigUint, BigUint};

fn main() {
    let mut fib : BigUint = one();
    let mut fib_prev : BigUint = one();
    let thresh = num::pow(10.to_biguint().unwrap(), 999);
    let mut i = 2;
    while fib < thresh {
        let tmp = fib.clone();
        fib = fib + fib_prev;
        fib_prev = tmp;
        i = i + 1;
    }
    println!("(i, length, fib) = ({}, {}, {})", i, format!("{}", fib).len(), fib);
    println!("Previous fib number length: {}", format!("{}", fib_prev).len());
}
