fn main() {
    let mut x = 0is;
    let mut a = 0us;
    let mut v = vec!(1is , 2 , 3 , 5 , 8 , 13 , 21 , 34 , 55);
    for _i in [0is..23is].iter() {
        let x = (v[v.len() - 2] + v[v.len() - 1]) as isize;
        v.push(x);
    }
    for _i in [0is..v.len() as isize].iter() {
        if v[a] % 2 == 0 { x += v[a]; }
        a += 1;
    } 
    println!("{:?}, {:?}", x, v);
    println!("{}, {:?}", x, v);
}
