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
