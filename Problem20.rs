extern crate num;
use num::{BigUint, Zero, One};
use num::bigint::{ToBigInt, RandBigInt};
use std::iter::AdditiveIterator;
fn main() {
    let mut e = 0us;
    let mut vec = vec![];
    let mut f0 = 100.to_bigint().unwrap();
    let mut f1 = 100.to_bigint().unwrap();
    for _ in (0..100) {
        if f0 == 1.to_bigint().unwrap() {
            break;
        }
        f0 = f0 -1.to_bigint().unwrap();
       
        f1 = f1 * f0.clone();
    }
    let mut a = f1.to_string();
    let b: Vec<_> = a.chars().collect();
    for _ in (0us..b.len()) {
        vec.push(b[e].to_string().parse::<i32>().unwrap());
        e+=1;
   }
   let mut it = vec.iter().map(|&x| x);
  println!("{:?}", it.sum()); 
}
