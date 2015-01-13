///idiomatic code
-use std::iter::AdditiveIterator;
fn main() {
-    let squares_sum = (1u32..101).map(|x| x * x).sum();
-    let sum = (1u32..101).sum();
-    let sum_squared = sum * sum;
-
-    println!("{}", sum_squared - squares_sum);
}
//Original Code
se std::num::Float;

fn main() {
    let mut a = 0.0;
    let mut sqa = 0is;
    let mut nat = 0.0;
    for _ in [0is..101is].iter() {
        sqa += a.powi(2) as isize;
        a += 1.0;
    }
    a = 0.0;
    for _ in [0is..100is].iter() {
        a += 1.0;
        nat += a;
    }
    println!("{}", nat.powi(2) as isize - sqa);
}
