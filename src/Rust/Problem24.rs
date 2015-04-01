fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for _ in (0us..999999us) { 
        v.next_permutation(); 
    }
    println!("The answer is :{:?}" , v);
}
