use std::num::Float;

fn main() {
    let mut a = 0.0;
    let mut sqa = 0is;
    let mut nat = 0.0;
    for _ in range(0i, 101i) {
    sqa += a.powi(2) as isize;
    a += 1.0;
}
a = 0.0;
    for _ in range(0i, 100i) {
    a += 1.0;
    nat += a;
    }
    println!("{}", nat.powi(2) as isize - sqa);
}
