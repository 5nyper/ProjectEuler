use std::iter::AdditiveIterator;

fn main() {
    let squares_sum = (1u32..101).map(|x| x * x).sum();
    let sum = (1u32..101).sum();
    let sum_squared = sum * sum;

    println!("{}", sum_squared - squares_sum);
}
