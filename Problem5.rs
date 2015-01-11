fn main(){
    let mut i = 1i32;
    loop {
        let mut divisible = true;
        for j in 2..21 {
            if i % j != 0 {
                divisible = false;
                break;
            }
        }

        if divisible {
            println!("{} is your answer", i);
            break;
        }

        i += 1;
    }
}
