//Idiomatic Code
use std::iter::AdditiveIterator;
fn main() {
    println!("{}", (0u32..1000).filter(|&x| (x % 3 == 0) || (x % 5 == 0)).sum());
}
fn main() {
    let mut x = 0is;
    let mut a = 0us;
    let v: Vec<isize> = range(0is, 1001is).collect();
    for _i in [0is..v.len() as isize].iter() {
        if v[a] % 5 == 0 || v[a] % 3 == 0 { x += v[a]; }
        a += 1;
    } 
    println!("{:?}, {:?}", x, v);
    println!("{}, {:?}", x, v);
}
