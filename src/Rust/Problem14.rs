fn main() {
    let mut a = 13is;
    let mut ph = 13is;
    let mut res = vec![a];
    loop {
        if a %2 == 0 {
            a /= 2;
            res.push(a);
            }
        else if a == 1 {
            ph += 1;
            a = ph; 
           if res.len() >= 500 && a -1 <= 1000000{  //Got 500 from trialing from 300, by one hundred
                println!("\x07");
                println!("Term: {}\nCollatz Sequence Length:{}\n\n\x07", a -1, res.len());
            } 
            res = vec![a];
        }
        else {
            a *= 3;
            a+=1;
            res.push(a);
        }
    }

}
