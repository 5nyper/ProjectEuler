extern crate num;
use num::{BigUint, Zero, One};
use num::bigint::{ToBigInt, RandBigInt};
use std::iter::AdditiveIterator;
use std::num::Float;

fn main() {
   let mut a = 0us;
   let b = num::pow(2.to_bigint().unwrap(), 1000);
   let v: Vec<_> = b.to_string().chars().collect();
   let mut vec = vec![];
   for _ in (0us..v.len()) {
    vec.push(v[a].to_string().parse::<i32>().unwrap());
    a+=1;
    }
   
   let it = vec.iter().map(|&x| x);
  println!("{}",it.sum()); 
}

