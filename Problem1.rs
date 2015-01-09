fn main() {
    let mut x: int = 0;
    let mut a: usize = 0;
    let mut v: Vec<int> = (range(0i, 1001i).collect());
    for _i in range(0i, v.len() as int) {
        if v[a] % 5 == 0 || v[a] % 3 == 0 { x += v[a]; }
        a += 1;
    } 
    println!("{}, {}", x, v);
}
