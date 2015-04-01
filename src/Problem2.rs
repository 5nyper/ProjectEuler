//Idiomatic Code
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
    let mut x = 0is;
    let mut a = 0us;
    let mut v = vec!(1is , 2 , 3 , 5 , 8 , 13 , 21 , 34 , 55);
    for _i in [0is..23is].iter() {
        let x = v[v.len() - 2] + v[v.len() - 1];
        v.push(x);
    }
    for _i in [0us..v.len()].iter() {
        if v[a] % 2 == 0 { x += v[a]; }
        a += 1;
   } 
    println!("{:?}, {:?}", x, v);
    println!("{}, {:?}", x, v);
}
