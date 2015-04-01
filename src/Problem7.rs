//Idiomatic code
use std::num::Float;

fn is_prime(x: i32) -> bool {
   if x < 2 { return false; }
    if x == 2 { return true; }
    for i in 2..((x as f64).sqrt().ceil() as i32 + 1) {
       if x % i == 0 { return false; }
   }
    true
}

fn main() {
    const N: i32 = 10_001;
    let mut count = 2i32;

   if  N >= 1 {
        println!("First {} prime numbers are:", N);
        println!("1: 2");
    }

    let mut i = 3i32;
    while count <= N {
        if is_prime(i) {
            println!("{}: {}", count, i);
            count += 1;
        }
        i += 1;
    }
}
   
