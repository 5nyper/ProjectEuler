//Idiomatic Code
 use std::iter::AdditiveIterator;
-use std::num::Float;
 
-fn is_prime(x: u64) -> bool {
-    if x < 2 { return false; }
-    if x == 2 { return true; }
-    for i in 2..((x as f64).sqrt().ceil() as u64 + 1) {
-        if x % i == 0 { return false; }
-    }
-    true
-}
-
-fn main() {
-    println!("{}", (2u64..2_000_000).filter(|&x| is_prime(x)).sum());
}
//Original Code
fn main() {
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
