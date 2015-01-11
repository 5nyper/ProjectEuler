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
    let mut num = 600851475143u64;
    let mut max = 0;

    let mut i = 2;
    while i <= num {
        if num % i == 0 && is_prime(i) {
            println!("{} is a prime factor", i);
            num /= i;
            if i > max { max = i; }
            i = 2;
        }

        i += 1;
    }

    println!("Largest prime factor: {}", max);
}
