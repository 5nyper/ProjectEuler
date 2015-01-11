//Fast code
fn main() {
    let mut i = 20is;
    loop {
        if i % 19 == 0 && i % 18 == 0 && i % 17 == 0 && i % 16 == 0 && i % 15 == 0 && i % 14 == 0
            && i % 13 == 0 && i % 12 == 0 && i % 11 == 0 {
                print!("result: {}", i);
                break;
            }
        i += 20;
    }
}
//BLAZING FAST CODE!
const BOUND: u64 = 20;
fn main() {
    let factors = (2..(BOUND+1)).collect::<Vec<_>>();
    let mut step = 2;

    for &f in factors.iter() {
        step = lcm(step, f);
    }

    let mut value = step;

    loop {
        if factors.iter().all(|&f| value % f == 0) {
            break;
        }

        value += step;
    }

    println!("{}", value);
}

#[inline]
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

#[inline]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a }
    else { gcd(b, a % b) }
}
//Idiomatic code
fn main(){
    let mut i = 1i32;
    loop {
        let mut divisible = true;
        for j in 2..21 {
            if i % j != 0 {
                divisible = false;
                break;
            }
        }

        if divisible {
            println!("{} is your answer", i);
            break;
        }

        i += 1;
    }
}
//Slower Original Code
fn main(){
    let mut ph = 1is;
    let mut i = 1is;
    while ph==1 {
        if i % 2 == 0 && i % 3 == 0 && i % 4 == 0 && i % 5 == 0 && i % 6 == 0 && i % 7 == 0 && i % 8 == 0 && i % 9 == 0 && i % 10 == 0 && i % 11 == 0 && i % 12 == 0 && i % 13 == 0 && i % 14 == 0 && i % 15 == 0 && i % 16 == 0 && i % 17 == 0 && i % 18 == 0 && i % 19 == 0 && i % 20 == 0 {
            println!("{} is your answer", i);
            ph = 2;
            }
    i += 1;
    }
}
